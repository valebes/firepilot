/*
 * Firecracker API
 *
 * RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying JSON modeled data. The transport medium is a Unix Domain Socket.
 *
 * The version of the OpenAPI document: 1.3.0
 * Contact: compute-capsule@amazon.com
 * Generated by: https://openapi-generator.tech
 */

/// BalloonUpdate : Balloon device descriptor.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BalloonUpdate {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    pub amount_mib: i32,
}

impl BalloonUpdate {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32) -> BalloonUpdate {
        BalloonUpdate { amount_mib }
    }
}
