use crate::common::SdlError;
use ash::{vk, InstanceError, LoadingError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BuildVulkanError {
    CreateEntryError(LoadingError),
    SdlError(SdlError),
    CreateInstanceError(InstanceError),
    VulkanError(vk::Result),
    CannotFindSuitableGPU,
    CannotFindSuitableQueue,
    CannotFindUsableFormat,
    CannotSetPresentMode,
}

impl From<LoadingError> for BuildVulkanError {
    fn from(load_err: LoadingError) -> Self {
        BuildVulkanError::CreateEntryError(load_err)
    }
}

impl From<SdlError> for BuildVulkanError {
    fn from(sdl_error: SdlError) -> Self {
        BuildVulkanError::SdlError(sdl_error)
    }
}

impl From<vk::Result> for BuildVulkanError {
    fn from(res: vk::Result) -> Self {
        BuildVulkanError::VulkanError(res)
    }
}

impl From<InstanceError> for BuildVulkanError {
    fn from(instance_error: InstanceError) -> Self {
        BuildVulkanError::CreateInstanceError(instance_error)
    }
}

impl Display for BuildVulkanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildVulkanError::CreateEntryError(loading) => {
                write!(f, "Building Vulkan failed! Create Entry Error: {}", loading)
            }
            BuildVulkanError::SdlError(sdl_error) => {
                write!(f, "Building Vulkan failed! Sdl Error : {}", sdl_error)
            }
            BuildVulkanError::CreateInstanceError(instance_error) => {
                write!(
                    f,
                    "Building Vulkan failed! Create Instance Error : {}",
                    instance_error
                )
            }
            BuildVulkanError::VulkanError(res) => {
                write!(f, "Building Vulkan failed! Vulkan Error : {}", res)
            }
            BuildVulkanError::CannotFindSuitableGPU => {
                write!(f, "Building Vulkan failed! Cannot find any suitable GPU.")
            }
            BuildVulkanError::CannotFindSuitableQueue => {
                write!(f, "Building Vulkan failed! Cannot find any suitable Queue.")
            }
            BuildVulkanError::CannotFindUsableFormat => {
                write!(f, "Building Vulkan failed! Cannot find any usable format.")
            }
            BuildVulkanError::CannotSetPresentMode => {
                write!(f, "Building Vulkan failed! Cannot set present mode.")
            }
        }
    }
}

impl Error for BuildVulkanError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BuildVulkanError::CreateEntryError(loading) => Some(loading),
            BuildVulkanError::SdlError(sdl_error) => Some(sdl_error),
            BuildVulkanError::CreateInstanceError(instance_error) => Some(instance_error),
            BuildVulkanError::VulkanError(res) => Some(res),
            _ => None,
        }
    }
}
