/// Type of balloon device
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MemballoonModel {
    None,
    Virtio,
    VirtioNonTransitional,
    VirtioTransitional,
}

/// A balloon device
///
/// # Examples
///
/// ```
/// use virtblocks::devices::Memballoon;
/// use virtblocks::devices::MemballoonModel;
///
/// let mut memballoon = Memballoon::new();
///
/// assert_eq!("", memballoon.to_str());
///
/// memballoon.set_model(MemballoonModel::Virtio);
/// assert_eq!("virtio-memballoon", memballoon.to_str());
/// ```
pub struct Memballoon {
    model: MemballoonModel,
}

impl Memballoon {
    pub fn new() -> Self {
        Self {
            model: MemballoonModel::None,
        }
    }

    pub fn set_model(&mut self, model: MemballoonModel) {
        self.model = model;
    }

    pub fn get_model(&self) -> MemballoonModel {
        self.model
    }

    pub fn to_str(&self) -> String {
        let mut ret = String::from("");

        match self.model {
            MemballoonModel::Virtio => {
                ret.push_str("virtio-memballoon");
            }
            MemballoonModel::VirtioNonTransitional => {
                ret.push_str("virtio-memballoon-non-transitional");
            }
            MemballoonModel::VirtioTransitional => {
                ret.push_str("virtio-memballoon-transitional");
            }
            MemballoonModel::None => {}
        }

        ret
    }
}

impl Default for Memballoon {
    fn default() -> Self {
        Self::new()
    }
}
