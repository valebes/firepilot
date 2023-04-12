/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MemoryBackend {
    #[serde(rename = "backend_type")]
    pub backend_type: BackendType,
    /// Based on 'backend_type' it is either 1) Path to the file that contains the guest memory to be loaded 2) Path to the UDS where a process is listening for a UFFD initialization control payload and open file descriptor that it can use to serve this process's guest memory page faults
    #[serde(rename = "backend_path")]
    pub backend_path: String,
}

impl MemoryBackend {
    pub fn new(backend_type: BackendType, backend_path: String) -> MemoryBackend {
        MemoryBackend {
            backend_type,
            backend_path,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BackendType {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Uffd")]
    Uffd,
}

impl Default for BackendType {
    fn default() -> BackendType {
        Self::File
    }
}
