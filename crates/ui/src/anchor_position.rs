use gpui::Corner;

/// Extended anchor position enum that includes center positions.
/// GPUI's Corner enum only has 4 variants (TopLeft, TopRight, BottomLeft, BottomRight),
/// but we need additional center positions for UI components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnchorPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    LeftCenter,
    RightCenter,
}

impl AnchorPosition {
    /// Convert to GPUI's Corner enum by mapping center positions to nearest corner.
    pub fn to_corner(self) -> Corner {
        match self {
            AnchorPosition::TopLeft => Corner::TopLeft,
            AnchorPosition::TopCenter => Corner::TopLeft,
            AnchorPosition::TopRight => Corner::TopRight,
            AnchorPosition::BottomLeft => Corner::BottomLeft,
            AnchorPosition::BottomCenter => Corner::BottomLeft,
            AnchorPosition::BottomRight => Corner::BottomRight,
            AnchorPosition::LeftCenter => Corner::TopLeft,
            AnchorPosition::RightCenter => Corner::TopRight,
        }
    }

    /// Get the opposite corner along the axis.
    pub fn other_side_corner_along(self, axis: gpui::Axis) -> Corner {
        self.to_corner().other_side_corner_along(axis)
    }
}

impl From<Corner> for AnchorPosition {
    fn from(corner: Corner) -> Self {
        match corner {
            Corner::TopLeft => AnchorPosition::TopLeft,
            Corner::TopRight => AnchorPosition::TopRight,
            Corner::BottomLeft => AnchorPosition::BottomLeft,
            Corner::BottomRight => AnchorPosition::BottomRight,
        }
    }
}

impl Default for AnchorPosition {
    fn default() -> Self {
        AnchorPosition::TopLeft
    }
}
