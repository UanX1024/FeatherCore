//! Device Tree framework for FeatherCore
//! FeatherCore 设备树框架
//! 
//! This module provides a device tree framework similar to Linux, allowing board configurations
//! to define hardware devices and their connections in a declarative way.
//! 此模块提供类似于 Linux 的设备树框架，允许板级配置以声明方式定义硬件设备及其连接。

use core::fmt;
use alloc::{string::String, vec::Vec, format};
use alloc::string::ToString;

/// Device tree node
/// 设备树节点
#[derive(Debug, Clone)]
pub struct DeviceTreeNode {
    /// Node name
    /// 节点名称
    pub name: String,
    /// Node path
    /// 节点路径
    pub path: String,
    /// Properties
    /// 属性
    pub properties: Vec<Property>,
    /// Children nodes
    /// 子节点
    pub children: Vec<DeviceTreeNode>,
}

/// Device tree property
/// 设备树属性
#[derive(Debug, Clone)]
pub struct Property {
    /// Property name
    /// 属性名称
    pub name: String,
    /// Property value
    /// 属性值
    pub value: PropertyValue,
}

/// Property value types
/// 属性值类型
#[derive(Debug, Clone)]
pub enum PropertyValue {
    /// String value
    /// 字符串值
    String(String),
    /// Integer value
    /// 整数值
    Integer(u64),
    /// Boolean value
    /// 布尔值
    Boolean(bool),
    /// Array of integers
    /// 整数数组
    IntegerArray(Vec<u64>),
}

/// Device tree parser
/// 设备树解析器
pub struct DeviceTreeParser;

impl DeviceTreeParser {
    /// Parse device tree from JSON
    /// 从 JSON 解析设备树
    pub fn from_json(json: &str) -> Result<DeviceTreeNode, String> {
        // TODO: Implement JSON parsing
        // TODO: 实现 JSON 解析
        Err("JSON parsing not implemented".to_string())
    }
    
    /// Parse device tree from DTS file
    /// 从 DTS 文件解析设备树
    pub fn from_dts(dts: &str) -> Result<DeviceTreeNode, String> {
        // 简单的 DTS 解析实现
        // 注意：这是一个简化版本，仅支持基本的 DTS 语法
        let mut root = DeviceTreeNode {
            name: "".to_string(),
            path: "/".to_string(),
            properties: Vec::new(),
            children: Vec::new(),
        };
        
        let mut current_path = "/".to_string();
        let mut current_node = &mut root;
        let mut stack = Vec::new();
        
        for line in dts.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                continue;
            }
            
            if line.starts_with("/") && line.ends_with("{") {
                // 根节点
                let name = line.strip_suffix(" {").unwrap_or(&line).trim_start_matches("/");
                root.name = if name.is_empty() { "root".to_string() } else { name.to_string() };
            } else if line.starts_with("}") {
                // 结束节点
                if let Some(parent) = stack.pop() {
                    current_node = parent;
                    current_path = current_path.rsplit_once('/').unwrap_or(("", "/")).0.to_string();
                    if current_path.is_empty() {
                        current_path = "/".to_string();
                    }
                }
            } else if line.ends_with("{") {
                // 开始新节点
                let name = line.strip_suffix(" {").unwrap_or(&line);
                let node_name = name.split('@').next().unwrap_or(name).to_string();
                let node_path = if current_path == "/" {
                    format!("/{}", node_name)
                } else {
                    format!("{}/{}", current_path, node_name)
                };
                
                let new_node = DeviceTreeNode {
                    name: node_name,
                    path: node_path.clone(),
                    properties: Vec::new(),
                    children: Vec::new(),
                };
                
                stack.push(current_node);
                current_node.children.push(new_node);
                current_node = current_node.children.last_mut().unwrap();
                current_path = node_path;
            } else if line.contains("=") {
                // 属性
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    continue;
                }
                
                let name = parts[0].trim().to_string();
                let value_str = parts[1].trim();
                
                let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                    // 字符串
                    PropertyValue::String(value_str.trim_matches('"').to_string())
                } else if value_str.starts_with('<') && value_str.ends_with('>') {
                    // 整数或整数数组
                    let content = value_str.trim_matches(&['<', '>'][..]);
                    let numbers: Vec<&str> = content.split_whitespace().collect();
                    if numbers.len() == 1 {
                        // 单个整数
                        if let Ok(num) = numbers[0].parse::<u64>() {
                            PropertyValue::Integer(num)
                        } else {
                            continue;
                        }
                    } else {
                        // 整数数组
                        let mut array = Vec::new();
                        for num_str in numbers {
                            if let Ok(num) = num_str.parse::<u64>() {
                                array.push(num);
                            }
                        }
                        PropertyValue::IntegerArray(array)
                    }
                } else if value_str == "true" {
                    // 布尔值 true
                    PropertyValue::Boolean(true)
                } else if value_str == "false" {
                    // 布尔值 false
                    PropertyValue::Boolean(false)
                } else {
                    // 其他类型，暂时作为字符串处理
                    PropertyValue::String(value_str.to_string())
                };
                
                current_node.properties.push(Property {
                    name,
                    value,
                });
            }
        }
        
        Ok(root)
    }
}

/// Device tree manager
/// 设备树管理器
pub struct DeviceTreeManager {
    /// Root node
    /// 根节点
    root: DeviceTreeNode,
}

impl DeviceTreeManager {
    /// Create a new device tree manager
    /// 创建一个新的设备树管理器
    pub fn new(root: DeviceTreeNode) -> Self {
        Self {
            root,
        }
    }
    
    /// Get root node
    /// 获取根节点
    pub fn root(&self) -> &DeviceTreeNode {
        &self.root
    }
    
    /// Find node by path
    /// 通过路径查找节点
    pub fn find_node(&self, path: &str) -> Option<&DeviceTreeNode> {
        self.find_node_recursive(&self.root, path)
    }
    
    /// Recursive node search
    /// 递归节点搜索
    fn find_node_recursive(&self, node: &DeviceTreeNode, path: &str) -> Option<&DeviceTreeNode> {
        if node.path == path {
            return Some(node);
        }
        
        for child in &node.children {
            if let Some(found) = self.find_node_recursive(child, path) {
                return Some(found);
            }
        }
        
        None
    }
    
    /// Get property value
    /// 获取属性值
    pub fn get_property(&self, node_path: &str, property_name: &str) -> Option<&PropertyValue> {
        self.find_node(node_path)
            .and_then(|node| node.properties.iter()
                .find(|p| p.name == property_name)
                .map(|p| &p.value))
    }
    
    /// Create device tree manager from generated device tree
    /// 从生成的设备树创建设备树管理器
    #[cfg(feature = "devicetree")]
    pub fn from_generated() -> Self {
        Self {
            root: crate::generated::devicetree::DEVICE_TREE.clone(),
        }
    }
}

/// Driver binding trait
/// 驱动绑定 trait
pub trait DriverBind {
    /// Bind driver to device tree node
    /// 将驱动绑定到设备树节点
    fn bind(node: &DeviceTreeNode) -> Result<Self, String> where Self: Sized;
}

/// Device tree error
/// 设备树错误
#[derive(Debug)]
pub enum DeviceTreeError {
    /// Node not found
    /// 节点未找到
    NodeNotFound,
    /// Property not found
    /// 属性未找到
    PropertyNotFound,
    /// Invalid property value
    /// 无效的属性值
    InvalidPropertyValue,
    /// Parsing error
    /// 解析错误
    ParsingError(String),
}

impl fmt::Display for DeviceTreeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceTreeError::NodeNotFound => write!(f, "Node not found"),
            DeviceTreeError::PropertyNotFound => write!(f, "Property not found"),
            DeviceTreeError::InvalidPropertyValue => write!(f, "Invalid property value"),
            DeviceTreeError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

/// Device tree helper functions
/// 设备树辅助函数
pub mod helper {
    use super::*;
    
    /// Get string property
    /// 获取字符串属性
    pub fn get_string_property(node: &DeviceTreeNode, name: &str) -> Result<String, DeviceTreeError> {
        node.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| match &p.value {
                PropertyValue::String(s) => Some(s.clone()),
                _ => None,
            })
            .ok_or(DeviceTreeError::PropertyNotFound)
    }
    
    /// Get integer property
    /// 获取整数属性
    pub fn get_integer_property(node: &DeviceTreeNode, name: &str) -> Result<u64, DeviceTreeError> {
        node.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| match &p.value {
                PropertyValue::Integer(i) => Some(*i),
                _ => None,
            })
            .ok_or(DeviceTreeError::PropertyNotFound)
    }
    
    /// Get boolean property
    /// 获取布尔属性
    pub fn get_boolean_property(node: &DeviceTreeNode, name: &str) -> Result<bool, DeviceTreeError> {
        node.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| match &p.value {
                PropertyValue::Boolean(b) => Some(*b),
                _ => None,
            })
            .ok_or(DeviceTreeError::PropertyNotFound)
    }
    
    /// Get integer array property
    /// 获取整数数组属性
    pub fn get_integer_array_property(node: &DeviceTreeNode, name: &str) -> Result<Vec<u64>, DeviceTreeError> {
        node.properties
            .iter()
            .find(|p| p.name == name)
            .and_then(|p| match &p.value {
                PropertyValue::IntegerArray(arr) => Some(arr.clone()),
                _ => None,
            })
            .ok_or(DeviceTreeError::PropertyNotFound)
    }
    

}