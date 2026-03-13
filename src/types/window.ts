// 窗口状态管理
export interface WindowState {
  isExpanded: boolean;
  isHoveringEdge: boolean;
  windowHeight: number;
  isDocked: boolean;
  isMinimized: boolean;
  previousHeight: number;
}