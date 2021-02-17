// comments with "ICON FIT" means < 10 characters
// arguments with $0 $1 etc

// general stuffs
yes "确认"
ok "好"
cancel "取消"
delete "删除"
X "X"
Y "Y"
Z "Z"
noSelectedMesh "对象未选择"
advancedSettings "高级设置"

// pbr
baseColor "颜色"
roughness "粗糙度"
metalness "金属强度"

// ------------------------------------------------------
// about
about.minify "全屏显示"
about.minify.help "在设备支持的情况下，可以通过四指触碰屏幕来开关全屏显示"
about.turntable "旋转展示"
about.turntableSpeed "旋转速度"
about.credits "鸣谢"
about.creditsOpenSource "开源项目"
about.creditsArts "MatCap与HDRI"

// ------------------------------------------------------
// alert
alert.confirmDelete "请确认是否删除？"
alert.confirmDelete.yes "确认删除"
alert.hole.nothing "该对象没有孔洞！"
alert.shape.notVisible "当前对象不可见！"
alert.trim.nothing "未找到可裁切对象"
alert.trim.full  "对象不能完全裁切"
alert.mask.noExtract "未找到可提取对象"
alert.mask.noSplit "未找到可分离对象"
alert.view.disabled "一些功能将在浏览模式下禁用："
alert.separate.fail "该对象只有一部分，所以无法分开"
alert.voxelRemesh.success "网格重构成功！"
alert.voxelRemesh.empty "网格重构失败，因为结果并未产生面。"
alert.voxelRemesh.invalidInput "输入无效！"
alert.matrix.clone "将复制此对象"
alert.gizmo.usePivot "使用自定义坐标原点"
alert.gizmo.useAuto "使用自动坐标原点"
alert.gizmo.editPivot "编辑坐标原点模式"
alert.gizmo.editObject "编辑对象模式"
alert.dynamic.enable "启用动态网格"
alert.dynamic.disable "关闭动态网格"
alert.colorPicker "在对象上拖动手指选取一个颜色"
alert.camera.resetView "重置视图"
alert.camera.snapView "切换视图"
alert.mask.show "显示蒙版"
alert.mask.hide "隐藏蒙版"
alert.selection.lock "锁定所选项"
alert.selection.unlock "解锁所选项"
alert.selection.isolate "隔离所选项"
alert.selection.showAll "显示全部"
alert.quickSave "正在自动保存..."
alert.multiresLost "模型细分将会丢失，是否继续？"
// autosave popup
alert.autoSave.auto "将在 $0s 后自动保存"
// bottom warning
alert.warning.needLayer "当前工具仅在活动图层上可用"
alert.warning.multiresLost "模型细分将会丢失"
alert.warning.paintingHidden "绘画已被隐藏，请在设置面板里将其打开。"
alert.warning.noPartialWireframe "打开线框显示时，局部绘画将被禁用。"
// bottom tip
alert.tip.polygonDot "轻点绿色图标以应用几何体。"
alert.tip.shapeOrthographic "为了避免因透视视图而产生的偏差，建议在相机设置里切换到正交视图。"
// undo
alert.state.trial "这是试用版本，您无法再撤销。"

// ------------------------------------------------------
// background
background "背景"
background.settings "设置"
background.color "颜色"
background.environment "环境"
background.blur "模糊"
background.exposure "曝光"

background.imageEnable "参考图像"
background.imageEnable "启用"
background.imageX "X轴方向"
background.imageY "Y轴方向"
background.imageRotation "旋转"
background.imageScale "缩放"
background.imageOverlay "透明度"
background.imageReset "重设"

// ------------------------------------------------------
// camera
camera "相机"
// saved views
camera.updateView "更新视图？"
camera.addView "添加视图"
camera.focusOn "正在观察"
// projection
camera.projection "视图"
camera.orthographic "正交视图"
camera.perspective "透视视图"
camera.fov "焦距"
// orbit
camera.orbit "视图旋转"
camera.orbit.help "旋转模式启用后可使用双指旋转视图。"
camera.trackball "旋转模式"
camera.turntable "水平模式"
// speed
camera.speed "相机速度"
camera.rotation "旋转"
camera.panning "平移"
camera.zooming "缩放"
// misc
camera.resetView "重置视图"
camera.snapView "固定视图"
// interaction
camera.pivot "视图中心点"
camera.doubleTapMesh "双击对象"
camera.doubleTapBackground "双击背景"
camera.doubleTapPivot "双击后改变"
camera.doubleTapPivot.help "双击后改变坐标视图中心点。"
camera.autoPivot "平移/缩放后改变"
camera.autoPivot.help "双指移动相机后，中心点会随之移动。"
camera.doubleTapFocus "聚焦"
camera.doubleTapFocus.help "双击物体表面后视图中心将移动至该点。"
camera.doubleTapFocusSelection "聚焦所选项"
camera.doubleTapFocusSelection.help "双击背景后相机将会缩放移动至最适合该对象的视图。"

// scene and layer lists
expandList "展开图标"
expandList.help "可以让菜单里的图标排列间距放大。"

// ------------------------------------------------------
// file
file.project.empty "您没有保存的项目"
file.project.unsaved "更改未保存！"
file.project.loseUnsaved "如不保存，您的更改将会丢失！"
file.project.lastManualSave "上一次手动保存的预览"
file.project.trialNoOpen "这是试用版本，您将无法重新打开当前项目！"
file.project.trialOnlyOpen "这是试用版本，您只能打开当前项目！"

// project
file.project "项目"
file.project.save "保存"
file.project.save.confirm "确认保存 $0？"
file.project.saveAs "另存为"
file.project.saveAs.confirm "确认覆盖 $0？"
file.project.open "打开"
file.project.open.confirm "将打开选定的项目 $0？"
file.project.add "添加"
file.project.add.confirm "确认添加 $0 至当前项目？"
file.project.new "新建"
file.project.new.confirm "确认新建场景？"
file.project.delete "删除"
file.project.delete.confirm "确认删除 $0？"
file.project.delete.confirmActive "删除 $0？\n\n这是当前正打开的项目！"
file.project.delete.confirmOk "确定要删除吗？"

// autosave
file.project.autoSave "自动保存项目"
file.project.autoSave.help "每隔一段时间将您的项目另存为一个单独的文件。

这个自动保存文件可以在以下目录找到：

$0"
file.project.autoSave.popup "弹窗提醒"
file.project.autoSave.minutes "自动保存间隔"
file.project.autoSave.delete "删除自动保存文件"
file.project.autoSave.delete.confirm "确认删除？"

// import
file.import.title "从外部导入"
file.import.title.help "支持导入的格式：\n- Wavefront (.obj)\n- glTF 2.0 (.glb .gltf)\n- STL (.stl)"
file.importOpen "打开"
file.importOpen.confirm "确定打开新文件？"
file.import.add "添加"
file.import.add.confirm "确定添加新文件？"

file.exportSelection "只导出选择部分"
file.exportSelection.help "只导出当前选择对象，而不是所有场景。"
file.convertToQuad "Reconstruct quad"
file.convertToQuad.help "Reconstruct quads from triangles by pairing triangle (if they are adjacent in the files)."

// export
file.export.title "导出"
file.export.title.help "建议导出 glTF 格式，因为它比其他格式支持更多属性。"

// gltf
file.export.gltf "导出 glTF 2.0"
file.export.gltfLayer "导出图层"
file.export.gltfLayer.help "将图层导出为可变体。这是来自官方的特性，能在更多软件上使用。"
file.export.gltfColor "导出颜色"
file.export.gltfColor.help "导出的是顶点颜色。这是来自官方的特性，能在更多软件上使用。"
file.export.gltfNormal "导出法线"
file.export.gltfNormal.help "如想在其他软件上打开该文件，请勾选此选项。\n该选项对本应用没有影响。"
file.export.gltfExtraPaint "导出其他"
file.export.gltfExtraPaint.help "将导出粗糙度、金属强度、蒙版和图层绘画。其他软件不会读取该属性。"

// obj
file.export.obj "导出 OBJ 格式"
file.export.objWarning "图层、粗糙度、金属强度、蒙版和绘画图层等其他属性将会丢失。"
file.export.objColorAppend "导出颜色"
file.export.objColorAppend.help "在顶点之后添加颜色信息\n\n只有部分3D软件能够识别。"
file.export.objColorHexa "十六进制颜色"
file.export.objColorHexa.help "像ZBrush那样将颜色转换为十六进制。\n\n只有部分3D软件能够识别。"

// stl
file.export.stl "导出 STL 格式"
file.export.stlWarning "图层和绘画将会丢失"
file.export.stlAscii "默认情况下，格式为二进制。\n\n您可以选择导出为文本格式（ASCII），但文件会更大。"

file.settings.title "设置"
file.settings.title.help "大部分应用的设置都保存在此处（相机界面等）。\n
某些资源将自动保存在其他地方，包括：
- HDR
- 材质
- 画笔形状
- 背景
- 项目\n
目前暂时无法保存画笔设置，自定义笔画已在开发计划中。"

// settings
file.settings.reset "恢复默认设置"
file.settings.reset.confirm "确定重设所有设置？\n\n项目、画笔形状、材质、HDRI与背景将不会被影响。"

// materials
file.materials "材质库"
file.materials.reset "重置为默认"
file.materials.reset.confirm "确定要重置材质库吗？"

// tools
file.tools "工具预设"
file.tools.reset "重置为默认"
file.tools.reset.confirm "确定要重置材质库吗？"

// render
file.render "渲染"
file.render.showInterface "显示操作界面"
file.render.size "渲染尺寸"
file.render.screenResolution "屏幕尺寸"
file.render.export "导出为png"
file.render.4kWarn "导出4K格式可能会使用大量内存，请确认文件保存之后再导出！"
file.render.transparent "导出透明背景"
file.render.transparent.help "打开此选项可以让您更方便地把渲染图导入到平面软件里。\n
暂不支持部分透明对象导出。"

// ------------------------------------------------------
// history
history "历史记录"
history.root "新建"
history.undoConfirm "您确定要撤销所有操作吗？"
history.undoWarning "如在此之后进行更改，将会覆盖之前的所有操作。"
history.stack "历史记录设置"
history.limitSize "历史记录限制 (Mb)"
history.limitSize.help "历史记录的最大大小（以Mb为单位）。\n
历史记录状态会随着下一个操作记录而改变。"
history.limitStack "历史记录步数"
history.limitStack.help "程序可保留的最大操作数量。\n
历史记录状态会随着下一个操作记录而改变。"
history.rangeProtect "历史记录保护范围"
history.rangeProtect.help "如您在历史记录中做了大量操作，程序会在覆盖操作之前提示您。"
history.gesture "快捷手势"
history.gesture.help "双指轻点撤销。\n\n三指轻点重做。"
history.restoreCamera "恢复相机视角"
history.restoreCamera.help "启用该选项后您可以在撤销或重做时同时恢复当时的相机视角。"
// display undo/redo
history.state.undo "撤销： $0"
history.state.redo "重做： $0"
history.state.symmetrySplit "镜像"
history.state.voxelRemesh "体素网格重构"
history.state.surfaceRemesh "表面网格重构"
// state multires
history.state.multiresToDynamic "模型细分转为动态网格"
history.state.multiresLevel "改变分辨率"
history.state.multiresSubdivide "细分网格"
history.state.multiresReverse "粗化网格"
history.state.multiresDeleteLower "删除低分辨率模型"
history.state.multiresDeleteHigher "删除高分辨率模型"
// mesh
history.state.meshDynamicToStatic "动态网格转为静态网格"
history.state.meshStaticToDynamic "静态网格转为动态网格"
history.state.meshSymmetryUpdate "改变对称"
history.state.meshMatrixUpdate "矩阵变换"
history.state.meshVisibility "可见性"
history.state.meshMaterial "改变材质"
// state scene
history.state.sceneAddRemove "场景"
history.state.sceneMeshOrder "模型顺序"
// state layer
history.state.layerOrder "更改图层顺序 $0"
history.state.layerMergeRedo "取消合并图层 $0"
history.state.layerCreate "添加图层 $0"
history.state.layerDelete "删除图层 $0";
history.state.layerMerge "合并图层 $0";
history.state.layerHide "隐藏图层 $0"
history.state.layerShow "显示图层 $0"
history.state.layerSelect "选择图层 $0"
history.state.layerUnselect "取消选择图层 $0"
history.state.layerName "图层 $0 重命名";
history.state.layerFactor "调整图层 $0 参数";
history.state.layerFactorOffset "调整图层 $0 偏移度";
history.state.layerFactorColor "调整图层 $0 透明度";
history.state.layerFactorRoughness "调整图层 $0 粗糙度";
history.state.layerFactorMetalness "调整图层 $0 金属强度";
// state light
history.state.lightVisible "调整灯光 $0 可见性"
history.state.lightIntensity "调整灯光 $0 强度"
history.state.lightColor "调整灯光 $0 颜色"
history.state.lightPosition "调整灯光 $0 位置"
history.state.lightShadow "调整灯光 $0 阴影"
history.state.lightBias "调整灯光 $0 阴影偏移"
history.state.lightAttachment "调整灯光 $0 定位方式"
history.state.lightAdd "添加灯光 light $0"
history.state.lightDelete "删除灯光 $0"
history.state.lightCopy "复制灯光 $0"
history.state.lightMove "移动灯光 $0"

// ------------------------------------------------------
// pressure menu
input.useGlobal "使用全局压感设置"
input.useGlobal.help "勾选后，所有工具都会使用相同的压感参数。\n
如您希望给此工具单独设定压感参数，请取消勾选"

input.pressure "压感"
input.pressureTitle "压感设置 ($0)"
input.pressure.noTool "此工具不适用压感设置。"
input.pressure.noGrab "此工具会忽略压感设置。"
input.pressure.radius "半径"
input.pressure.intensity "强度" 
input.pressure.useRadius "启用"
input.pressure.useIntensity "启用" 
input.pressure.curveRadius "半径"
input.pressure.curveIntensity "强度"

input.cameraInteraction "相机移动："
input.sculptInteraction "雕刻:"
input.interaction.fingerAndStylus "手指与触控笔"
input.interaction.finger "手指"
input.interaction.stylus "触控笔"

input.fingerSmooth "将手指用于平滑"
input.unknownPressure "允许未识别的压感"
input.unknownPressure.help "当您的触控笔压感无法使用或者希望使用手指压感时，请勾选此选项。" 
// pencil
input.pencilAction.none "无"
input.pencilAction.smooth "平滑"
input.pencilAction.alt "添加或减去"
input.pencilAction.android "触控笔按钮"
input.pencilAction.android.help "实验功能"
input.pencilAction.ios "双击Pencil"
input.pencilAction.ios.help "仅支持Apple Pencil 第二代"
// size rejection
input.useSizeRejection "启用忽略尺寸"
input.useSizeRejectionWarning "如果您的手指无法操控，请退出并重启Nomad。\n此选项每次重启后都会关闭。"
input.useSizeRejectionConfirm "请确保您的文件妥善保存后再点击确认！"
input.useSizeRejection.help "如果手指与屏幕的接触面积超过设定值，屏幕将忽略手指的本次操作。\n
部分设备可能不支持此选项"
input.sizeRejection "尺寸阈值"
// help
input.interaction.title "交互选项" 
input.interaction.title.help "以下选项均为全局设置。"

// ------------------------------------------------------
// interface
interface "界面设置"

// bottom buttons
interface.bottomButtons "底部按钮"
interface.quickVoxelRemesh "体素网格重构"
interface.quickWireframe "网格开关"
interface.quickLockSelection "锁定选择"
interface.quickLockSelection.help "启用后，您无法通过点击方式选择对象。"
interface.quickCameraReset "重置视图"
interface.quickCameraSnap "切换视图"
interface.quickCameraSnapFlip "翻转基本视图"
interface.quickCameraSnapFlip.help "当相机处于基本视图时，点击切换视图将会翻转至背面。"

// left
interface.leftButtons "左边按钮"
interface.quickSmooth "平滑"
interface.quickMask "蒙版"
interface.quickToggle "反向操作锁定"
interface.quickPaint "材质"
interface.quickAlpha "画笔形状"
interface.maskGesture "蒙版手势"
interface.screenTooSmall "如设备屏幕太小，一些图标将不会显示。"
interface.maskGesture.help "按住蒙版快捷方式，并且：\n
- 在背景上拖动可清除蒙版
- 在背景上点击可反相蒙版
- 在对象上点击可锐化蒙版边界"


// colors
interface.colors "界面颜色"
interface.colorSelect "主色"
interface.colorBase "底色"
interface.colorBaseTransparent "面板颜色" 
interface.panelTransparent "面板透明度"
interface.blurFactor "模糊强度"

// color preset
interface.colorsPresets "界面预设"
interface.presetBlurRed "红"
interface.presetBlurBlue "蓝"
interface.presetBlurGreen "绿"
interface.presetBlurYellow "黄"
interface.presetBlackWhite "黑与白"
interface.presetWhiteBlack "白与黑"
interface.presetLividOrange "青与橙"
interface.presetCardboard "纸板"
interface.presetDefault "默认设置"

// style
interface.style "菜单风格"
interface.resetAll "重置界面设置"
interface.resetAll.confirm "确定要重置界面吗？"
interface.flipTop "整体反转"
interface.flipBottom "反转底部图标"
interface.flipMiddle "反转中间图标"
interface.autoClose "操作时收起菜单"
interface.autoClose.help "点击背景与对象时自动收起菜单"
interface.showTooltips "显示工具提示"
interface.showTooltips.help "你在点的这个小问号就是工具提示 :-D"
interface.materialPreview "调整材质参数预览"
interface.toolboxHide "自动隐藏工具栏"
interface.toolboxHide.help "如果你想隐藏工具栏，请勾选此选项。"
interface.toolboxMaxColumn "工具栏列数"
interface.rounding "界面圆角"
interface.inlined "滑块紧凑"
interface.dampingSlider "降低滑块灵敏度"
interface.dampingSlider.help "勾选此选项可让滑块在调节参数时更加精确。"
interface.curveToolSymmetric "压力曲线对称"
interface.curveToolSymmetric.help "使笔刷设置里的衰减参数曲线对称。"
interface.animated "动效"
interface.scale "界面缩放"
interface.cursorStep "垂直间距"
interface.panelWidth "面板宽度"
interface.fontScale "字体尺寸"

// toolbox
interface.toolsOrder "工具顺序"
interface.openOrderTools "编辑"
interface.resetOrderTools "重设"
interface.resetOrderTools.confirm "确认要重设顺序吗？"

// debug
interface.debug "Debugging"
interface.debug.warning "For debugging only!"

// ------------------------------------------------------
// layer sub menu
layer.action "操作"
layer.name "重命名"
layer.delete "删除图层"
layer.move "移动图层"
layer.duplicate "复制图层"
layer.mergeDown "向下合并"
layer.factors "通道参数"
layer.offsetFactor "位置偏移"
layer.colorFactor "颜色浓度"

// ------------------------------------------------------
// layers menu
layers.addLayer "添加图层"
layers.addLayerTrial "试用版本只能给每个对象添加一个图层！"
layers.title "图层"
layers.title.help "图层能够记录位置偏移和绘画，这对于非线性工作流程来说非常有用。
例如，通过试验不同的面部表情而不依赖于历史记录来撤消更改。

图层是从上到下排序的，所以上方的图层会遮盖下方的图层。

为了解决图层的不透明性，图层的所有通道（颜色浓度、粗糙度、金属强度）都会使用相同的蒙版。
您可以使用橡皮工具来擦除当前图层上的绘画蒙版。";
layers.multipleObjectWarning "您选择了多个对象，无法修改图层。"
layers.primitive "基本体无法添加图层。"
layers.baseSelected "无"

// ------------------------------------------------------
// light sub menu
light "Light"
light.color "Color"
light.intensity "Intensity"
light.attachment "Attachment"
light.attachment.fixed "Fixed"
light.attachment.camera "Camera"
light.attachment.environment "Environment"
light.attachment.help "-- Fixed
Light orientation won't change.\n
-- Camera
Light orientation depends on the camera view."
light.shadowCast "Shadow"
light.shadowNormalBias "Normal bias"

// ------------------------------------------------------
// material
material "材质"
material.addNew "添加新材质"
material.matcapWarning "粗糙度与金属强度在材质捕捉模式下不可用。"
material.opacity = "透明度"

// ------------------------------------------------------
// menu name (visible on small screen menu are collapsed)
menu.files "Files"
menu.scene "Scene"
menu.topology "Topology"
menu.render "Render"
menu.postProcess "PostProcess"
menu.camera "Camera"
menu.background "Background"
menu.tool "Tool"
menu.stroke "Stroke"
menu.paint "Paint"
menu.symmetry "Symmetry"
menu.pressure "Pressure"
menu.layers "Layers"
menu.settings "Settings"
menu.interface "Interface"
menu.history "History"
menu.historySettings "Settings"
menu.about "About"

// ------------------------------------------------------
// mesh sub menu
mesh.action "操作"
mesh.closeHoles "封闭孔洞"
mesh.separate "分离对象"
mesh.triplanarWarning "图层、绘画与模型细分将会丢失。"
mesh.triplanarResolution "分辨率"
mesh.triplanarCubic "强制转换为立方体"
mesh.triplanarConvert "转换"
mesh.name "名称"
mesh.type "类型"
mesh.typeStatic "静态模型"
mesh.typeMultiresolution "模型细分"
mesh.typeDynamic "动态模型"

// ------------------------------------------------------
// painting
paint.useGlobal "应用全局材料"
paint.useGlobal.help "如勾选此选项，其他工具的材质也将会与所选材质相同。"
paint.usePainting "启用" 
paint.useColor "颜色" 
paint.useRoughness "粗糙度" 
paint.useMetalness "金属强度"
paint.intensity "画笔强度"
paint.paintAll "全部上色" 
paint.paintAll.help "将当前材料应用到所选对象上。"
paint.paintAllForce "强制全部上色"
paint.paintAllForce.help "将当前材料应用到所选对象上。\n
蒙版区域与未勾选通道也会被应用。"
paint.strokePaintingTitle "画笔设置 ($0)"
paint.layerWarning "图层上的通道蒙版不可用。"

// ------------------------------------------------------
// postprocess
postprocess.mainEnable "后期处理" 
// fxaa
postprocess.fxaaEnable "抗锯齿（FXAA）"
// ssr
postprocess.ssrEnable "屏幕空间反射（SSR）" 
postprocess.ssrFactor "强度" 
postprocess.ssrDistanceFading "淡化距离" 
postprocess.ssrDistanceFading.help "根据反射距离来减弱效果
此选项能减弱SSR所产生的伪影。"
postprocess.ssrMatcapWarning "SSR仅在PBR渲染模式下有效。"
// ssao
postprocess.ssaoEnable "环境光遮蔽" 
postprocess.ssaoRadius "半径" 
postprocess.ssaoFactor "强度" 
postprocess.ssaoBias "曲率偏移" 
postprocess.ssaoBias.help "效果的敏感性取决于表面曲率。"
// dof
postprocess.dofEnable "景深"
postprocess.dofBlurFar "远景模糊" 
postprocess.dofBlurNear "近景模糊"
postprocess.dofFocusTip "点击对象表面可以改变焦点。"
// bloom
postprocess.bloomEnable "泛光效果" 
postprocess.bloomIntensity "强度" 
postprocess.bloomRadius "半径" 
postprocess.bloomRadius.help "此参数可调节泛光的宽度。"
postprocess.bloomThreshold "阈值" 
postprocess.bloomThreshold.help "泛光阈值能够判断一个像素是否使产生泛光效果。
如果该值为零，会使所有像素都产生泛光效果。"
// tone mapping
postprocess.toneEnable "色调映射" 
postprocess.toneExposure "曝光" 
postprocess.toneContrast "对比度" 
postprocess.toneSaturation "饱和度" 
postprocess.toneMappingNone "无"
// chromatic
postprocess.chromaticEnable "色彩偏移" 
postprocess.chromaticFactor "强度" 
// vignette
postprocess.vignetteEnable "晕影" 
postprocess.vignetteSize "半径" 
postprocess.vignetteHardness "硬度" 
// sharpness
postprocess.sharpnessEnable "锐化" 
postprocess.sharpnessFactor "强度" 
// grain
postprocess.grainEnable "噪点" 
postprocess.grainFactor "强度" 

// ------------------------------------------------------
// primitive (scene menu)
primitive "基本体"
primitive.box "立方体"
primitive.sphere "球体"
primitive.sphereUV "UV球体"
primitive.icosahedron "宝石"
primitive.cylinder "圆柱"
primitive.cone "圆锥"
primitive.torus "圆环"
primitive.plane "平面"
primitive.triplanar "三向投影"
primitive.needValidate "基本体需转换为可编辑对象后才可雕刻。"

primitive.mainConfig "范围"
primitive.topology "拓扑参数"
primitive.geometry "几何"

// common config
primitive.mirror "镜像"
primitive.mirror.help "通过镜像方式复制该基本体。"
primitive.validate "转换"
primitive.maxFaces "面数限制"
primitive.maxFaces.help "该基本体可拥有的最大面数。\n
该参数只能在转换为可编辑对象前修改。"
primitive.linear "线性细分"
primitive.subdivision "细分等级"

// common config
primitive.size "尺寸"
primitive.sizeX "X"
primitive.sizeY "Y"
primitive.sizeZ "Z"
primitive.division "分段数"
primitive.divisionX "X"
primitive.divisionY "Y"
primitive.divisionZ "Z"
primitive.angleX "X"
primitive.angleY "Y"
primitive.angleZ "Z"
primitive.constantDensity "固定比例"
primitive.projectOnSphere "投影在球体上"
primitive.projectOnSphere.help "将点分布在一个完美的球体上。"

// triplanar
primitive.triplanar.title "三向投影 - 设置"
primitive.triplanar.title.help "三向投影是将原对象三个平面的投影重新组合起来填充体素网格，然后将其多边形化。\n
您可以通过在三个投影上修改蒙版或移动滑块的方式来改变几何体。\n
建议您关闭对称选项，否则可能会导致最终效果与您的预期不符。\n
您可以开启遮罩面板中的“拓扑连接”选项来在绘制时影响其他平面。"
primitive.triplanarIsolate "可见性"
primitive.triplanarSameSize "固定比例（立方体）"
primitive.triplanarPolish "羽化"
primitive.triplanarResetMask "重置蒙版"
primitive.triplanarReproject "重新投影"
primitive.triplanarReproject.title "修改三向投影设置后会更新平面上的蒙版。\n
If you uncheck this option, it will revert to the default spherical masks."
primitive.isolate.all "全部"
primitive.isolate.back "背面"
primitive.isolate.right "右面"
primitive.isolate.bottom "底面"
// plane
primitive.planeSameSize "固定比例（平面）"
// box
primitive.boxRegular "固定比例（立方体）"
// icosahedron
primitive.icosahedronRadius "半径尺寸"
// torus
primitive.torusRadiusOuter "外圈半径"
primitive.torusRadiusInner "内圈半径"
primitive.torusAngle "角度"
primitive.torusHole "孔洞"
// sphere
primitive.sphereRadius "半径尺寸"
// cylinder
primitive.cylinderSameSize "固定两个圆面的半径"
primitive.cylinderRadiusBottom "底面半径"
primitive.cylinderRadiusTop "顶面半径"
primitive.cylinderHeight "高度"
primitive.cylinderHole "具有孔洞"
// cone
primitive.coneRadius "半径"
primitive.coneHeight "高度"

// common resources stuffs
resource.delete "删除"
resource.import "导入"

// ------------------------------------------------------
// scene
scene.title "场景"
scene.title.help "使用选择复选框时，按住并拖动手指即可轻松选择其他对象。 "
scene.mergeSimple "简单合并"
scene.mergeVoxel "体素合并"
scene.voxelResolution "分辨率"
scene.subtractionTip "相减运算：隐藏减去对象后点击体素合并。"
scene.intersectionTip "相交运算：隐藏所有相关模型后点击体素合并。"

// ---------------------------
// settings
settings.displayTitle "显示设置"
settings.fingerRotateLighting "三指旋转灯光"
settings.fingerRotateLighting.help "在屏幕上使用三指水平移动可使环境、灯光与材质捕捉旋转。"
// wireframe
settings.wireframeTitle "对象网格设置"
settings.wireframeDisplay "显示对象网格"
settings.wireframeColor "对象网格颜色"
// backface
settings.backfaceTitle "双面显示设置"
settings.backfaceVisible "双面显示"
settings.backfaceVisible.help "打开双面显示可以让您看到模型的“内面”。\n
所有的三角形或四边形都朝向一个特定的方向，例如在基本球体上，所有的面都朝向外部。\n
如果您将相机移动到球体内部，这些面就是背面。"
settings.backfaceColor "内面颜色"
settings.backfaceColored "内面着色"
// outline
settings.outlineTitle "轮廓"
settings.outlineEnable "显示轮廓"
settings.outlineThickness "粗细"
settings.outlineColor "颜色"
// grid
settings.gridTitle "世界网格"
settings.gridDisplay "显示世界网格"
settings.gridHeight "高度"
settings.gridColor "颜色"
// cursor
settings.cursorWhileSculpting "雕刻时显示画笔"
settings.cursorShowDot "显示指针点"
settings.cursorShowDot.help "指针点会在您移动相机和雕刻时显示。"
settings.cursorShowRope "显示画笔准星"
// other
settings.renderRatio "实时渲染分辨率"
settings.darkenUnselected "变暗未选中对象"
settings.smoothShading "平滑阴影"
settings.partialDraw "局部雕刻"
settings.partialDraw.help "功能未完善！\n
仅建议您在雕刻高精度细小模型时使用。\n
它能让雕刻过程更加流畅，但不建议您打开显示对象网格。\n
启用此功能可能会在使用画笔时产生一些奇怪的东西。"
settings.partialDrawWarning "如果显示不正常，请不要忘记关闭此选项！"
settings.detailRangeTitle "体素/动态网格重构"
settings.detailRange "最大细节范围"
settings.detailRange.help "此选项可调节体素和动态网格详细程度的最大值。\n
详细程度越高会使模型产生更多的多边形，请斟酌调整！"
settings.showPainting "显示场景绘画"
settings.lightIcon "显示灯光图标"
settings.lightIcon.help "在屏幕上显示灯光图标，这样您可以直接选择并编辑灯光。"
settings.showSnapCube "Snap cube"
settings.loadGuiSettings "加载项目GUI设置"
settings.loadGuiSettings.help "当您打开或导入项目文件时，同时加载项目中包含的GUI设置。"
settings.holeTitle "填补孔洞"
settings.holeNonManifold "填补非流形孔洞"
settings.holeNonManifold.help "应用将会尝试填补非流形孔洞。\n
此选项不会被保存在设置中。\n"
settings.keepImportTopology "在导入时保留拓扑"
settings.keepImportTopology.help "如您不希望应用破坏导入模型拓扑，请勾选此选项。\n
应用将不会：\n - 重新排序顶点和面\n - 删除重叠顶点和面\n - 移除未使用顶点\n"
// multires
settings.multiresTitle "模型细分"
settings.multiresMaxVertices "最大顶点数量"
settings.multiresMaxVertices.help "应用在模型细分之前并不会检查内存，多边形数量过多很容易会导致应用崩溃。"
settings.multiresLowResVertices "最低分辨率阈值"
settings.multiresLowResVertices.help "在您移动相机时，模型对象可能会以较低分辨率显示。\n
如您希望显示更高的分辨率，可以增加此值。"
// stat
settings.showSceneStats "现实场景状态"
settings.statNone "不显示"
settings.statSelection "显示所选项"
settings.statAll "显示全部"
// experimental
settings.experimentalTitle "实验性功能"
settings.notSaved "这些选项不会在设置中保存。"
// settings.parallel "Parallel sculpting"

// ------------------------------------------------------
// shading
shading "渲染模式"
// lights
shading.lights "灯光"
shading.lights.addLight "添加灯光"
shading.lights.matcapWarning "灯光选项在材质捕捉模式下不可用。"
// environment
shading.environment "环境"
shading.environmentImport "导入HDR"
shading.environmentExposure "曝光"
shading.environmentRotation "旋转"
shading.environmentRotation.help "在屏幕上使用三指水平移动可使环境、灯光与材质捕捉旋转。"
shading.environmentAttachedToCamera "与相机固定"
shading.environmentAttachedToCamera.help "将环境与相机固定。\n
这能让光线保持固定，对于雕刻来说非常有用。"
// matcap
shading.matcap "材质捕捉"
shading.matcapRotation "旋转"
shading.matcapRotation.help "在屏幕上使用三指水平移动可使环境、灯光与材质捕捉旋转。"
shading.matcapGlobal "使用全局材质捕捉。"
shading.matcapGlobal.help "如果您希望在不同对象上使用不同的材质捕捉，请取消此选项。"

// ------------------------------------------------------
// bottom shortcut buttons (ICON FIT)
shortcut.voxel "重构"
shortcut.wire "网格"
shortcut.mask "蒙版"
shortcut.reset "重设"
shortcut.snap "切换"
shortcut.solo "隔离"
shortcut.lock "锁定"

// ------------------------------------------------------
// stat
stat.ramScene "场景"
stat.vramScene "显存场景"
stat.ramHistory "历史"
stat.vramRender "显存渲染"
stat.ramOther "其他"
stat.usedMemory "已用内存："
stat.freeMemory "剩余内存："
stat.total "总计："
stat.used "已使用："
stat.free "剩余："
stat.faces "面数："
stat.triangles "三角面："
stat.vertices "顶点："
stat.quads "四边形："
stat.sceneFaces "场景面数："
stat.sceneVertices "场景顶点数："

// ------------------------------------------------------
// stroke
stroke "笔刷"
strokeTitle "笔刷设置 ($0)"
stroke.useWorldRadius "恒定笔刷半径"
stroke.useWorldRadius.help "勾选后笔刷半径将不会随着视图的缩放而改变。\n
此选项将会影响到所有工具。"
stroke.useShareRadius "相同笔刷半径"
stroke.useShareRadius.help "使所有工具的笔刷半径相同。"
stroke.minSpacingAdjustIntensity "调整间隔强度"
stroke.minSpacingAdjustIntensity.help "调整笔刷强度，以保证根据笔画间距产生一定的变化。"
stroke.minSpacing "笔刷间距"
stroke.minSpacing.help "每个笔画之间的距离，与笔画半径有一定的相关性。\n
将该值调低可使笔刷显得更加顺滑，但也会影响性能。"
stroke.lazySmooth "顺滑笔画"
stroke.lazySmooth.help "通过平均计算多个点来获得更加平滑的笔画。\n
将该值调高，会使笔画不跟手。"
stroke.lazyRadius "笔刷落后"
stroke.lazyRadius.help "笔画将会按一定的距离落后于指针位置。\n
此功能可用于绘制平滑线条。"
stroke.globalSettings "这是一个全局设置。"
stroke.snapRadius "续接笔画范围"
stroke.snapRadius.help "如果落笔处在最后一笔笔刷附近，会自动续接上。\n
此功能可用于绘制长线条，但需要频繁停顿时。"
stroke.sculptOffset "笔刷偏移"
stroke.sculptOffset.help "使笔刷持续偏移于触控处\n
此功能适用于小屏幕设备。在使用时，手指不会遮挡到屏幕。"
stroke.accumulate "叠加笔画"
stroke.accumulate.help "如启用此选项，则每个笔画可添加或减去的数量将不会有限制。"
stroke.useDynamicTopology "允许动态拓扑"
stroke.connectedTopology "连接拓扑"
stroke.connectedTopology.help "启用此选项后，画笔将会只雕刻连接到所选表面的顶点。\n
此选项一般适用于移动工具, 例如专门移动与另一零件自相交的零件。 "
stroke.onlyFrontFace "只影响对象表面"
stroke.onlyFrontFace.help "打开此选项后，应用会忽略对背面的操作。\n
该功能可帮助您在不影响另一侧的情况下在几何平面上绘画。\n
该选项也可用于雕刻，但您可能会遇到一些不便。"
stroke.intensityMultiplier "笔刷强度放大"
stroke.curveFalloff "衰减"
stroke.onlyLasso "该设置仅对套索工具有效。"
// alpha
stroke.alpha "形状" 
stroke.alphaInvert "形状反相"
stroke.alphaScale "形状缩放"
stroke.alphaScale.help "该值最小时,笔刷形状方框与半径圆内接。\n
该值最大时，笔刷形状方框与半径圆外切。"
// stroke type
stroke.strokeType "笔刷类型"
stroke.strokeTypeDot "点"
stroke.strokeTypeDrag "拖拽"
stroke.strokeTypeGrab "抓取"
stroke.strokeTypeGrabRadius "抓取 - 可调半径"
stroke.strokeTypeGrabIntensity "抓取 - 可调强度"

// ------------------------------------------------------
// symmetry
symmetry "对称"
symmetry.enable "启用对称"
symmetry.toolIgnore "当前工具不适用对称。"
// method
symmetry.method "对称类型"
symmetry.method.help " - 本地对称
可使用轴向变换或自由变换等工具移动调整对称平面。\n
 - 世界对称
对称平面是固定不动的。"
symmetry.methodWorld "世界对称"
symmetry.methodLocal "本地对称"
// mirror
symmetry.mirror "镜像"
symmetry.mirror.help "尝试在不影响拓扑的情况下重新应用对称。\n
拓扑必须对称且至少有一个边缘恰好位于对称平面上才能成功应用。\n
如果镜像失败，将会建议您强制对称。但这样会影响到拓扑。"
symmetry.apply "应用镜像"
symmetry.flip "翻转方向"
symmetry.flip.help "勾选此选项可更改投影面的方向。"
symmetry.applyFail "对称失败：
- $0\n
是否使用镜像来强制对称？";
// reset
symmetry.resetOrigin "重设对称中心"
symmetry.resetCenterMesh "对象中心"
symmetry.resetCenterWorld "世界中心"
symmetry.resetDirection "重设镜像平面"
// advanced
symmetry.showLine "显示线条"
symmetry.showPlane "显示平面"
symmetry.editWarning "编辑镜像平面是实验性功能。"
symmetry.edit "轴向变换"
symmetry.edit.help "您可以自由改变镜像平面。\n
此功能并未完善，请尽量不要使用。"

// ------------------------------------------------------
// tools
// left bar generic (ICON FIT)
tool.sliderDegree "角度 $0 °"
tool.sliderRadius "半径 $0 %"
tool.sliderIntensity "强度 $0 %"
tool.dynTopo "动态拓扑"
tool.symmetry "对称"
tool.mirror "镜像"
tool.clay "黏土"
tool.clay.sub "反向"
tool.brush "标准"
tool.move "移动"
tool.move.normal "法线方向"
tool.drag "拖拽"
tool.smooth "平滑"
tool.smooth.relax "规整网格"
tool.mask "蒙版"
tool.mask.unmask "消除蒙版"
tool.maskSelector "选择蒙版"
tool.paint "绘画"
tool.paint.erase "橡皮"
tool.smudge "涂抹"
tool.flatten "铲平"
tool.flatten.fill "填充"
tool.layer "层"
tool.crease "褶皱"
tool.trim "裁切"
tool.split "分割"
tool.project "投射"
tool.inflate "膨胀"
tool.pinch "挤捏"
tool.nudge "触碰"
tool.stamp "印记"
tool.clearLayer "擦除"
tool.gizmo "轴向变换"
tool.gizmo.auto "自动原点"
tool.gizmo.editPivot "编辑原点"
tool.gizmo.local "轴向"
tool.transform "自由变换"
tool.transform.move "移动"
tool.transform.rotate "旋转"
tool.transform.scale "缩放"
tool.measure "测量"
tool.view "浏览模式"
tool.shape.flip "翻转"
tool.shape.view "浏览"
tool.shape.lasso "套索"
tool.shape.curve "曲线"
tool.shape.polygon "多边形"
tool.shape.rectangle "矩形"
tool.shape.ellipse "椭圆"
tool.shape.line "直线"
// title
tool.settingsTitle "设置 ($0)"
// clay
tool.clay.flattenOffset "展平偏移 "
// crease
tool.crease.pinchFactor "力度"
// layer
tool.layer.removeInfluence "Use current layer offset"
tool.layer.removeInfluence.help "This option is only active when there is a current layer selected.\n
It will use the current layer offset to limit the displacement over strokes."
tool.layer.noLayerSelected "此选项仅在选择图层后可用。"
// flatten
tool.flatten.planeLock "锁定平面"
// smooth
tool.smooth.stickyBorder "Sticky vertex on border"
// masking
tool.mask.clear "清除"
tool.mask.invert "反相"
tool.mask.blur "模糊"
tool.mask.sharpen "锐化"
tool.mask.thickness "抽壳厚度"
tool.mask.polish "平滑边界"
tool.mask.extract "抽壳"
tool.mask.split "分离"
tool.mask.closeMask "分离操作（蒙版区域）："
tool.mask.closeUnmask "分离操作（非蒙版区域）："
tool.mask.closeAction "分离操作："
tool.mask.closeActionNone "无"
tool.mask.closeActionFill "填补"
tool.mask.closeActionShell "抽壳"
tool.mask.closeAction.help " - 无
仅分离蒙版区域，并且不封闭对象。

 - 填补
孔洞会被填补并光滑。
不要在平面上使用。

-- Shell
通过增加厚度的方式来封闭图形。"
// matrix
tool.matrix "坐标"
tool.matrix.clone "克隆"
tool.matrix.action "操作"
tool.matrix.action.help " - 返回原点
将对象移回原位。

 - 重设
重设对象的所有变换。

-- 烘焙
重新记录对象变换后的顶点坐标。在视觉上什么都不会改变。"
tool.matrix.transformOperation "变换操作"
tool.matrix.translation "位移"
tool.matrix.rotation "旋转"
tool.matrix.scale "缩放"
tool.matrix.uniformScale "等比缩放"
tool.matrix.uniformScale.help "应用不支持非等比缩放的对象变换，因此将用顶点变换替代。"
tool.matrix.moveToOrigin "返回原点"
tool.matrix.resetTransform "重设"
tool.matrix.bakeTransform "烘焙"
tool.matrix.bakeTransform.confirm "变换结果将会被烘焙在活动图层上。\n\n您确定要继续吗？"
tool.matrix.applyMethod "模式："
tool.matrix.applyMethodAuto "自动选择"
tool.matrix.applyMethodVertex "基于顶点"
tool.matrix.applyMethodObject "基于对象"
tool.matrix.applyMethod.help " - 自动选择：
让应用自动选择两种模式。
一般会选择基于对象模式，除非打开了对称或者在对象上有蒙版。

 - 基于顶点：
顶点坐标会独自变换。
该变换包括对称与蒙版变换。
如果变换的是基本体，将会强制使用基于对象模式。

 - 基于对象：
对象会整体变换。
不会变换对称与蒙版。
如果进行非等比缩放，将强制使用基于顶点模式。"
// transform
tool.transform.multiTouch "多点触控"
tool.transform.multiTouch.help "如果您禁用此选项，则每次都只能使用移动、旋转、缩放一种操作。"
// gizmo
tool.gizmo.size "部件尺寸"
tool.gizmo.autoHide "自动隐藏"
tool.gizmo.tap "Move custom pivot on single-tap"
tool.gizmo.tap.help "This option is only effective in custom pivot mode (Auto disabled).\n
-- None
Nothing happen when tapping on the mesh.

-- First hit
Move the gizmo on the first intersection.

-- Middle stab
Move the gizmo on the average of the first two intersections."
tool.gizmo.tapNone "None"
tool.gizmo.tapFirstHit "First hit"
tool.gizmo.tapMiddleStab "Middle stab"
// trim
tool.hole "Hole filling"
tool.hole.fillHoles "Fill holes"
// tool.hole.reproject "Reproject filled holes"
// tool.hole.reproject.help "Try to reproject the filled hole so that it follows more closely the cut.
// However, it will only work for rather simple projection."
tool.hole.bridges "Screen-space boolean"
tool.hole.bridges.help "If this option is enabled, you can punch holes in the volume.
The cut slope will also follow more closely the cutting shape."
tool.hole.threshold "Threshold epsilon"
tool.hole.threshold.help "Tweaking this value might help with the hole filling algorithm."
tool.hole.smoothing "Hole smoothing"
// smudge
// tool.smudge.projectScreen 
// tool.smudge.projectScreen.help "Smudge relies heavily on polygon density.\n
// Use this option if you want consistent smudge performance by projecting only once at the beginning of the stroke."
tool.smudge.fullProject "Project once"
tool.smudge.fullProject.help "You can make the smudge stroke faster by projecting the mesh only once at the beginning of the stroke.\n
If you don't move the camera between your smudge strokes, the projection can be avoided as well.\n
This setting is ignored if dynamic topology is activated."
tool.smudge.quality "Quality"
tool.smudge.quality.help "It changes the resolution of the projected pixels, lower values means faster strokes."
// trim / split / project / selMask
tool.shape "Shape"
tool.shape.rectangleSquare "Square"
tool.shape.rectangleCentered "Centered"
tool.shape.ellipseCircle "Circle"
tool.shape.ellipseCentered "Centered"
tool.shape.lineRotateStep "Rotate step"
// measure
tool.measure.goldenRatio "Show golden ratio"
// fallback
tool.noSettings "This tool doesn't have any specific settings."

// ------------------------------------------------------
// topology
topology "Topology"
// multires
topology.multires.title "Multiresolution"
topology.multires.title.help "Keep multiple resolution of a mesh.\n
If you make changes in a lower resolution, details from the higher resolutions will be reprojected when you switch back.\n
Layers are available on every resolution."
topology.multiresReverse "Reverse"
topology.multiresReverse.confirm "Could not create base subdivision.\n
The current topoloy is probably not a result from a subdivision."
topology.multiresReverse.confirm.yes "ok"
topology.multiresReverse.confirm.cancel ""
topology.multiresSubdivide "Subdivide"
topology.multiresSubdivideConfirm "The mesh will have $0M vertices, are you sure?"
topology.multiresDeleteLower "Delete lower"
topology.multiresDeleteHigher "Delete higher"
topology.multiresKeepTriangles "Keep triangles"
topology.multiresLinear "Flat subdivision"
// voxel
topology.voxel.title "Voxel remeshing"
topology.voxel.title.help "Remeshing by sampling the mesh on a grid.\n
If the object is not closed (watertight), an hole-filling algorithm will be applied first.\n
Layers are reprojected after remeshing but the quality will degrade."
topology.voxelResolution "Resolution"
topology.voxelRemesh "Remesh"
// dynamic topology
topology.surfaceUniform "Remesh"
topology.surfaceDetail "Detail"
topology.surfaceDetail.help "Unlike voxel remeshing, surface remeshing doesn't require the mesh to be closed.\n
It can also support masking so that you can protect some part of the mesh from topology changes.\n
Layers are updated correctly."
topology.surfaceMethod "Method"
toplogy.surfaceMethodUniformisation "Uniformisation"
toplogy.surfaceMethodSubdivision "Subdivision"
toplogy.surfaceMethodDecimation "Decimation"
topology.surfaceMethod.help "Behavior of dynamic topology:
- Uniformisation: add and remove detail
- Subdivision: add detail
- Decimation: remove detail"
topology.surfaceUseMasking "Protect masked area"
topology.surfaceUseMasking.help "The masked areas will protect the topology from beging changed."
topology.surfaceExtrapolate "Vertex extrapolation"
// dynamic
topology.dynamic "Dynamic topology"
topology.dynamicActivate "Enabled"
topology.dynamicActivate.help "With dynamic topology, sculpting tools can subdivide or simplify the mesh locally in real time.\n
This feature can have a noticeable impact on performance.\n
Layers are updated correctly."
topology.dynamicDetailLevel "Detail"
topology.dynamicDetailEdge "Detail"
topology.dynamicDetailMethod "Level of detail based on..."
topology.dynamicDetailMethodZoom "Zoom"
topology.dynamicDetailMethodRadius "Radius"
topology.dynamicDetailMethodConstant "Constant"
topology.dynamicDetailMethod.help "-- Zoom
The level of detail is based on how far you are from the surface.

-- Radius
The tool radius defines the amount of detail.

-- Constant
The detail is fixed, the detail value is shared with the voxel slider as well."
topology.dynamicQuality "Prefer..."
topology.dynamicQuality.help "If you choose Quality, the 2 main differences are:
- refinement is applied before the sculpting operator, you will get less interpolating artefact when painting or sculpting very small details
- refinement is not applied incrementally, if you sculpt very small details or do quick strokes, the topology will always be correctly refined\n
For better performance, and if you plan on using this option, you might consider enabling the \"partial drawing\" option in the Settings panel."
topology.dynamicQualitySpeed "Speed"
topology.dynamicQualityQuality "Quality"
topology.dynamicUsePressure "Use pressure on radius"
topology.dynamicUsePressure.help "Use this option if you want the pen pressure impact on tool radius to impact the level of detail."
topology.dynamicBrush "Brush"
topology.dynamicGlobal "Global"
topology.dynamicSettings "Settings - Brush / Global"

// ------------------------------------------------------
// version trial
version.buyWeb "网页版仅供演示"
version.buyFull "购买完整版本"
version.trialLimit "试用版本限制：
- 仅允许3次以内的撤销或重做
- 每个物体仅允许添加一个图层
- 仅允许启用一个项目
- 不允许导入和导出"
version.restorePurchase "恢复购买"
version.fullFeatures "- 撤消或重做不受限制
- 图层数量不受限制
- 允许保存和载入
- 可以导入和导出文件"
