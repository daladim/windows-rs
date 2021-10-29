#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Storage_Cabinets")]
pub mod Cabinets;
#[cfg(feature = "Win32_Storage_CloudFilters")]
pub mod CloudFilters;
#[cfg(feature = "Win32_Storage_Compression")]
pub mod Compression;
#[cfg(feature = "Win32_Storage_DataDeduplication")]
pub mod DataDeduplication;
#[cfg(feature = "Win32_Storage_DistributedFileSystem")]
pub mod DistributedFileSystem;
#[cfg(feature = "Win32_Storage_EnhancedStorage")]
pub mod EnhancedStorage;
#[cfg(feature = "Win32_Storage_FileHistory")]
pub mod FileHistory;
#[cfg(feature = "Win32_Storage_FileServerResourceManager")]
pub mod FileServerResourceManager;
#[cfg(feature = "Win32_Storage_FileSystem")]
pub mod FileSystem;
#[cfg(feature = "Win32_Storage_Imapi")]
pub mod Imapi;
#[cfg(feature = "Win32_Storage_IndexServer")]
pub mod IndexServer;
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
pub mod InstallableFileSystems;
#[cfg(feature = "Win32_Storage_IscsiDisc")]
pub mod IscsiDisc;
#[cfg(feature = "Win32_Storage_Jet")]
pub mod Jet;
#[cfg(feature = "Win32_Storage_OfflineFiles")]
pub mod OfflineFiles;
#[cfg(feature = "Win32_Storage_OperationRecorder")]
pub mod OperationRecorder;
#[cfg(feature = "Win32_Storage_Packaging")]
pub mod Packaging;
#[cfg(feature = "Win32_Storage_ProjectedFileSystem")]
pub mod ProjectedFileSystem;
#[cfg(feature = "Win32_Storage_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_Storage_Vhd")]
pub mod Vhd;
#[cfg(feature = "Win32_Storage_VirtualDiskService")]
pub mod VirtualDiskService;
#[cfg(feature = "Win32_Storage_Vss")]
pub mod Vss;
#[cfg(feature = "Win32_Storage_Xps")]
pub mod Xps;
