#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AlgoId {
    MaxRects,
    Guillotine,
    BottomLeft,
    NfpGreedy,
    SA,
    GA,
    SVGNest,
}

impl AlgoId {
    pub fn label(&self) -> &'static str {
        match self {
            AlgoId::MaxRects => "MaxRects 最大矩形",
            AlgoId::Guillotine => "Guillotine 断头台",
            AlgoId::BottomLeft => "Bottom-Left 天际线",
            AlgoId::NfpGreedy => "NFP 临界多边形贪心",
            AlgoId::SA => "SA 模拟退火",
            AlgoId::GA => "GA 遗传算法",
            AlgoId::SVGNest => "SVGNest 算法",
        }
    }

    pub fn short(&self) -> &'static str {
        match self {
            AlgoId::MaxRects => "MaxRects",
            AlgoId::Guillotine => "Guillotine",
            AlgoId::BottomLeft => "Bottom-Left",
            AlgoId::NfpGreedy => "NFP贪心",
            AlgoId::SA => "模拟退火SA",
            AlgoId::GA => "遗传算法GA",
            AlgoId::SVGNest => "SVGNest",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            AlgoId::MaxRects | AlgoId::Guillotine | AlgoId::BottomLeft => "通用/快速",
            AlgoId::NfpGreedy => "极限利用率",
            AlgoId::SA => "中等规模优化",
            AlgoId::GA => "大规模搜索",
            AlgoId::SVGNest => "复杂图形约束",
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            AlgoId::MaxRects => "BSSF贪心，维护所有自由矩形，工业级标准算法",
            AlgoId::Guillotine => "模拟锯切，每刀产生两个矩形区域，贴近实际切割",
            AlgoId::BottomLeft => "图形靠左下紧凑放置，维护天际线高度图",
            AlgoId::NfpGreedy => "精确计算No-Fit Polygon边界，从所有接触点选最优",
            AlgoId::SA => "以概率接受较差解跳出局部最优，在MaxRects基础上搜索更优排列",
            AlgoId::GA => "OX交叉+精英保留，使用MaxRects评估适应度，搜索全局最优排列",
            AlgoId::SVGNest => "GA进化顺序 + NFP精确接触定位 + 最低重心评分",
        }
    }

    pub fn has_meta(&self) -> bool {
        matches!(self, AlgoId::SA | AlgoId::GA | AlgoId::SVGNest)
    }

    pub fn p1_label(&self) -> &'static str {
        match self {
            AlgoId::SA => "初始温度",
            _ => "种群大小",
        }
    }

    pub fn p2_label(&self) -> &'static str {
        match self {
            AlgoId::SA => "迭代步数",
            _ => "进化代数",
        }
    }

    pub fn default_p1(&self) -> f64 {
        match self {
            AlgoId::SA => 1000.0,
            AlgoId::GA => 30.0,
            AlgoId::SVGNest => 20.0,
            _ => 20.0,
        }
    }

    pub fn default_p2(&self) -> f64 {
        match self {
            AlgoId::SA => 500.0,
            AlgoId::GA => 100.0,
            AlgoId::SVGNest => 60.0,
            _ => 80.0,
        }
    }
}
