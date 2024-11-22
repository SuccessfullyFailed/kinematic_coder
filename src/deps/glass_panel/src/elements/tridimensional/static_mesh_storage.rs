use crate::tridimensional::model::Mesh;
use std::error::Error;
use std::sync::Mutex;


// A static storage for meshes that stops large 3d models from having to be loaded at the same time.
static mut MESH_STORAGE:Mutex<Vec<Option<StaticMesh>>> = Mutex::new(Vec::new());
pub struct StaticMesh {
	source:String,
	mesh:Mesh,
	usages:usize
}
impl StaticMesh {

	/// Create a new StaticMesh or return the link to an existing mesh. Returns the index of the mesh in the storage list.
	pub fn create(source:&str, mesh:Mesh) -> StaticMeshLink {
		unsafe {

			// Find already existing item.
			if let Some(source_id) = StaticMesh::find_source(source) {
				MESH_STORAGE.lock().unwrap()[source_id].as_mut().unwrap().usages += 1;
				return StaticMeshLink::new(source_id);
			}

			// Create new item.
			StaticMesh::force_create(source, mesh)
		}
	}

	/// Create a new StaticMesh. Returns the index of the mesh in the storage list.
	pub fn force_create(source:&str, mesh:Mesh) -> StaticMeshLink {
		unsafe {
			let id:usize = MESH_STORAGE.lock().unwrap().len();
			MESH_STORAGE.lock().unwrap().push(Some(StaticMesh {
				source: source.to_owned(),
				mesh,
				usages: 1
			}));
			StaticMeshLink::new(id)
		}
	}

	/// Create a new StaticMesh from an OBJ file. Results the index of the mesh in the storage list.
	pub fn from_obj(file:&str) -> Result<StaticMeshLink, Box<dyn Error>> {
		Ok(StaticMesh::create(&format!("OBJ:{file}"), Mesh::from_obj(file)?))
	}

	/// Get the index of an existing mesh from tis source.
	pub fn find_source(source:&str) -> Option<usize> {
		unsafe {
			MESH_STORAGE.lock().unwrap().iter().position(|mesh_opt|
				match mesh_opt {
					Some(mesh) => mesh.source == source,
					None => false
				}
			)
		}
	}

	/// Get the memory size of the storage. 'size_of_val' does not appear to account for pointers.
	#[allow(dead_code)]
	pub fn memory_size() -> usize {
		unsafe {
			MESH_STORAGE.lock().unwrap().iter().map(|mesh_opt|
				match mesh_opt {
					Some(static_mesh) => static_mesh.source.len() + 8 + (static_mesh.mesh.vertices().len() * 12) + (static_mesh.mesh.faces().len() * 24),
					None => 0
				}
			).sum()
		}
	}
}

/// Allows communication with the static mesh. Automatically deletes static meshes that are not in use anymore.
#[derive(Debug, PartialEq)]
pub struct StaticMeshLink {
	id:usize
}
impl StaticMeshLink {

	/// Create a new link from an id.
	pub fn new(id:usize) -> StaticMeshLink {
		StaticMeshLink {
			id
		}
	}
	
	/// Clone and get the mesh.
	pub fn get_mesh(&self) -> Result<Mesh, Box<dyn Error>> {
		unsafe {
			if let Some(mesh_ref) = MESH_STORAGE.lock()?[self.id].as_ref() {
				Ok(mesh_ref.mesh.clone())
			} else {
				Err(format!("Mesh with id {} does not exist anymore", self.id).into())
			}
		}
	}
}
impl Drop for StaticMeshLink {
	fn drop(&mut self) {
		unsafe {

			// When out of scope, decrease usage count.
			let mut usages:usize = 0;
			if let Some(mesh) = MESH_STORAGE.lock().unwrap()[self.id].as_mut() {
				mesh.usages -= 1;
				usages = mesh.usages;
			}

			// When mesh has no active links anymore, remove the mesh from memory.
			if usages == 0 {
				MESH_STORAGE.lock().unwrap()[self.id] = None;
			}
		}
	}
}
impl Clone for StaticMeshLink {
	fn clone(&self) -> StaticMeshLink {
		unsafe { MESH_STORAGE.lock().unwrap()[self.id].as_mut().unwrap().usages += 1; }
		StaticMeshLink::new(self.id)
	}
}