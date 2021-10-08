# Todo List
* 为xecs添加一个SystemGroup的功能，将相关的system绑定到一起，减少引擎端用户代码量
* render::vulkan 添加交换链重建功能
* render::vulkan 调整架构
* imgui 添加剪贴板的支持
* imgui 优化Buffer的使用
* imgui 交换链重建之后需要重建渲染管线和帧缓存
* imgui 添加对自定义字体的支持
* imgui 修改渲染方式，imgui不再接管屏幕，将imgui的渲染结果绘制到离屏缓冲中
* 添加2d渲染功能，接管屏幕，可用于做Post-Processing、呈现imgui的绘制结果、呈现3d渲染结果
* 添加3d渲染功能
* 尝试优化text_input
* 添加音频系统
* 添加更多的todo