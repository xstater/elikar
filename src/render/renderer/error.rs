use ash::vk;
use std::io;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::any::TypeId;

#[derive(Debug)]
pub enum BuildRendererError {
    VulkanError(vk::Result),
    IoError(io::Error)
}

impl From<vk::Result> for BuildRendererError {
    fn from(vulkan : vk::Result) -> Self {
        BuildRendererError::VulkanError(vulkan)
    }
}

impl From<io::Error> for BuildRendererError {
    fn from(io_err: io::Error) -> Self {
        BuildRendererError::IoError(io_err)
    }
}

impl Display for BuildRendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildRendererError::VulkanError(vk_res) => {
                write!(f,"Build Renderer failed! Vulkan error : {}",vk_res)
            }
            BuildRendererError::IoError(io_err) => {
                write!(f,"Build Renderer failed! Cannot open file :{}",io_err)
            }
        }
    }
}

impl Error for BuildRendererError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildRendererError::VulkanError(vk_err) => Some(vk_err),
            BuildRendererError::IoError(io_err) => Some(io_err)
        }
    }
}