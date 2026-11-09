//! Strongly typed, deterministic fixtures: no Windows process APIs or external access.
#[derive(Clone, Copy, Debug)]
pub struct Process {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub cpu: f32,
    pub memory_mb: u32,
    pub disk_mb: f32,
    pub state: &'static str,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortBy { Cpu, Memory, Name }
impl SortBy {
    pub fn title(self) -> &'static str {
        match self { Self::Cpu => "CPU", Self::Memory => "内存", Self::Name => "名称" }
    }
    pub fn next(self) -> Self {
        match self { Self::Cpu => Self::Memory, Self::Memory => Self::Name, Self::Name => Self::Cpu }
    }
}
pub const PROCESSES: &[Process] = &[
    Process { id: 4260, name:"Explorer", description:"Windows 资源管理器", category:"系统", cpu: 3.2, memory_mb: 212, disk_mb:0.4, state:"运行中" },
    Process { id: 9184, name:"Code", description:"Visual Studio Code", category:"开发", cpu: 18.7, memory_mb: 1248, disk_mb:2.4, state:"运行中" },
    Process { id: 14320, name:"Browser", description:"浏览器演示进程", category:"应用", cpu: 24.6, memory_mb: 1860, disk_mb:1.2, state:"运行中" },
    Process { id: 5528, name:"Terminal", description:"命令行终端", category:"开发", cpu: 1.8, memory_mb: 146, disk_mb:0.1, state:"运行中" },
    Process { id: 2216, name:"Search Indexer", description:"系统索引服务", category:"系统", cpu: 6.1, memory_mb: 352, disk_mb:3.1, state:"后台" },
    Process { id: 10832, name:"Design Preview", description:"GPUI 界面预览", category:"开发", cpu: 8.5, memory_mb: 468, disk_mb:0.3, state:"运行中" },
    Process { id: 7740, name:"File Sync", description:"演示同步服务", category:"后台", cpu: 0.9, memory_mb: 105, disk_mb:1.8, state:"后台" },
    Process { id: 6904, name:"Media Player", description:"演示播放器", category:"应用", cpu: 4.7, memory_mb: 322, disk_mb:0.7, state:"运行中" },
    Process { id: 1108, name:"System", description:"系统内核", category:"系统", cpu: 2.3, memory_mb: 98, disk_mb:1.1, state:"运行中" },
];
pub fn filtered_sorted(query: &str, sort: SortBy) -> Vec<Process> {
    let query = query.trim().to_lowercase();
    let mut rows: Vec<Process> = PROCESSES.iter().copied().filter(|p|
        query.is_empty()
            || p.name.to_lowercase().contains(&query)
            || p.description.to_lowercase().contains(&query)
            || p.category.to_lowercase().contains(&query)
            || p.id.to_string().contains(&query)
    ).collect();
    rows.sort_by(|a,b| match sort {
        SortBy::Cpu => b.cpu.total_cmp(&a.cpu),
        SortBy::Memory => b.memory_mb.cmp(&a.memory_mb),
        SortBy::Name => a.name.cmp(b.name),
    });
    rows
}
pub const CPU_SERIES: &[f32] = &[23.,29.,26.,34.,39.,35.,28.,44.,50.,43.,55.,47.,36.,40.,52.,46.,38.,49.,54.,46.,57.,51.,44.,58.,50.,47.,53.,41.];
pub const MEMORY_SERIES: &[f32] = &[62.,61.,63.,62.,65.,66.,64.,67.,66.,69.,70.,68.,71.,72.,70.,71.,73.,72.,75.,73.,74.,76.,76.,74.,75.,76.,75.,77.];
pub const DISK_SERIES: &[f32] = &[17.,12.,29.,19.,43.,25.,30.,18.,14.,39.,22.,45.,27.,20.,36.,28.,19.,32.,41.,22.,16.,29.,35.,20.,23.,17.,27.,22.];
pub const NET_SERIES: &[f32] = &[9.,13.,16.,24.,17.,28.,42.,22.,29.,32.,47.,33.,37.,45.,34.,27.,38.,31.,48.,36.,40.,52.,35.,42.,30.,39.,53.,46.];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_and_sort_work() {
        let rows=filtered_sorted("code", SortBy::Cpu);
        assert_eq!(rows.len(),1);
        assert_eq!(rows[0].name,"Code");
        let all=filtered_sorted("",SortBy::Cpu);
        assert!(all.windows(2).all(|w|w[0].cpu>=w[1].cpu));
    }
}
