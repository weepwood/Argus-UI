//! Deterministic synthetic fixtures for reusable process and resource views.
#[derive(Clone, Copy, Debug)]
pub struct Process {
    pub id: u32,
    /// Synthetic parent identifier; used only for demonstrating tree layout.
    pub parent_pid: Option<u32>,
    pub name: &'static str,
    pub executable: &'static str,
    pub category: &'static str,
    pub cpu: f32,
    pub memory_mb: u32,
    pub disk_read_kb: u32,
    pub disk_write_kb: u32,
    pub network_kb: u32,
    pub gpu: f32,
    pub state: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortBy {
    Cpu,
    Memory,
    Name,
}

impl SortBy {
    pub fn title(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "内存",
            Self::Name => "名称",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Cpu => Self::Memory,
            Self::Memory => Self::Name,
            Self::Name => Self::Cpu,
        }
    }
}

pub const PROCESSES: &[Process] = &[
    Process {
        id: 4260,
        parent_pid: Some(8844),
        name: "画图",
        executable: "mspaint.exe",
        category: "应用",
        cpu: 0.1,
        memory_mb: 134,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 16071,
        parent_pid: Some(10028),
        name: "Argus",
        executable: "argus.exe",
        category: "应用",
        cpu: 0.2,
        memory_mb: 18,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 15612,
        parent_pid: Some(8844),
        name: "Git Bash",
        executable: "git-bash.exe",
        category: "应用",
        cpu: 0.1,
        memory_mb: 93,
        disk_read_kb: 6,
        disk_write_kb: 119,
        network_kb: 127,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 7324,
        parent_pid: Some(8844),
        name: "Microsoft Edge",
        executable: "msedge.exe",
        category: "应用",
        cpu: 11.0,
        memory_mb: 2672,
        disk_read_kb: 29,
        disk_write_kb: 576,
        network_kb: 15984,
        gpu: 18.0,
        state: "运行中",
    },
    Process {
        id: 17962,
        parent_pid: Some(8844),
        name: "Notepad++",
        executable: "notepad++.exe",
        category: "应用",
        cpu: 0.3,
        memory_mb: 112,
        disk_read_kb: 16,
        disk_write_kb: 326,
        network_kb: 344,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 2204,
        parent_pid: Some(8844),
        name: "QQ",
        executable: "QQ.exe",
        category: "应用",
        cpu: 0.1,
        memory_mb: 473,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 10736,
        parent_pid: Some(8844),
        name: "Visual Studio Code",
        executable: "Code.exe",
        category: "应用",
        cpu: 1.4,
        memory_mb: 1340,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 2.0,
        state: "运行中",
    },
    Process {
        id: 8844,
        parent_pid: None,
        name: "Windows 资源管理器",
        executable: "explorer.exe",
        category: "应用",
        cpu: 0.2,
        memory_mb: 167,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "运行中",
    },
    Process {
        id: 18283,
        parent_pid: Some(8844),
        name: "计算器",
        executable: "Calculator.exe",
        category: "后台进程",
        cpu: 0.0,
        memory_mb: 20,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "已暂停",
    },
    Process {
        id: 18372,
        parent_pid: Some(15612),
        name: "控制台窗口主进程",
        executable: "conhost.exe",
        category: "后台进程",
        cpu: 0.2,
        memory_mb: 3,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "后台",
    },
    Process {
        id: 10028,
        parent_pid: Some(15612),
        name: "cargo",
        executable: "cargo.exe",
        category: "后台进程",
        cpu: 0.3,
        memory_mb: 32,
        disk_read_kb: 36,
        disk_write_kb: 703,
        network_kb: 740,
        gpu: 0.0,
        state: "后台",
    },
    Process {
        id: 18076,
        parent_pid: Some(8844),
        name: "Everything",
        executable: "Everything.exe",
        category: "后台进程",
        cpu: 0.0,
        memory_mb: 190,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "后台",
    },
    Process {
        id: 13729,
        parent_pid: Some(10736),
        name: "Node.js JavaScript Runtime",
        executable: "node.exe",
        category: "后台进程",
        cpu: 0.4,
        memory_mb: 84,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 0.0,
        state: "后台",
    },
    Process {
        id: 13577,
        parent_pid: Some(10736),
        name: "rust-analyzer",
        executable: "rust-analyzer.exe",
        category: "后台进程",
        cpu: 2.4,
        memory_mb: 1016,
        disk_read_kb: 5,
        disk_write_kb: 100,
        network_kb: 107,
        gpu: 0.0,
        state: "后台",
    },
    Process {
        id: 1440,
        parent_pid: None,
        name: "Desktop Window Manager",
        executable: "dwm.exe",
        category: "Windows 进程",
        cpu: 1.8,
        memory_mb: 156,
        disk_read_kb: 0,
        disk_write_kb: 0,
        network_kb: 0,
        gpu: 5.0,
        state: "系统",
    },
    Process {
        id: 248,
        parent_pid: None,
        name: "System",
        executable: "System",
        category: "Windows 进程",
        cpu: 0.6,
        memory_mb: 12,
        disk_read_kb: 0,
        disk_write_kb: 48,
        network_kb: 0,
        gpu: 0.0,
        state: "系统",
    },
];

pub fn filtered_sorted(query: &str, sort: SortBy) -> Vec<Process> {
    let query = query.trim().to_lowercase();
    let mut rows: Vec<Process> = PROCESSES
        .iter()
        .copied()
        .filter(|process| {
            query.is_empty()
                || process.name.to_lowercase().contains(&query)
                || process.executable.to_lowercase().contains(&query)
                || process.category.to_lowercase().contains(&query)
                || process.id.to_string().contains(&query)
        })
        .collect();
    rows.sort_by(|a, b| match sort {
        SortBy::Cpu => b.cpu.total_cmp(&a.cpu),
        SortBy::Memory => b.memory_mb.cmp(&a.memory_mb),
        SortBy::Name => a.name.cmp(b.name),
    });
    rows
}


/// Build a stable parent-first view from already sorted, filtered rows.
///
/// Children whose parents are hidden by search become top-level rows. A second
/// pass also handles malformed/cyclic fixtures without losing records.
pub fn tree_rows(rows: &[Process]) -> Vec<(Process, usize)> {
    fn append(
        process: Process,
        depth: usize,
        rows: &[Process],
        seen: &mut Vec<u32>,
        output: &mut Vec<(Process, usize)>,
    ) {
        if seen.contains(&process.id) {
            return;
        }
        seen.push(process.id);
        output.push((process, depth));
        for child in rows.iter().copied().filter(|child| child.parent_pid == Some(process.id)) {
            append(child, depth + 1, rows, seen, output);
        }
    }

    let mut seen = Vec::with_capacity(rows.len());
    let mut output = Vec::with_capacity(rows.len());
    for process in rows.iter().copied() {
        if process.parent_pid.is_none()
            || !rows.iter().any(|candidate| Some(candidate.id) == process.parent_pid)
        {
            append(process, 0, rows, &mut seen, &mut output);
        }
    }
    for process in rows.iter().copied() {
        append(process, 0, rows, &mut seen, &mut output);
    }
    output
}

pub const CPU_SERIES: &[f32] = &[
    23., 29., 26., 34., 39., 35., 28., 44., 50., 43., 55., 47., 36., 40., 52., 46., 38., 49., 54.,
    46., 57., 51., 44., 58., 50., 47., 53., 41.,
];
pub const MEMORY_SERIES: &[f32] = &[
    62., 61., 63., 62., 65., 66., 64., 67., 66., 69., 70., 68., 71., 72., 70., 71., 73., 72., 75.,
    73., 74., 76., 76., 74., 75., 76., 75., 77.,
];
pub const DISK_SERIES: &[f32] = &[
    17., 12., 29., 19., 43., 25., 30., 18., 14., 39., 22., 45., 27., 20., 36., 28., 19., 32., 41.,
    22., 16., 29., 35., 20., 23., 17., 27., 22.,
];
pub const NET_SERIES: &[f32] = &[
    9., 13., 16., 24., 17., 28., 42., 22., 29., 32., 47., 33., 37., 45., 34., 27., 38., 31., 48.,
    36., 40., 52., 35., 42., 30., 39., 53., 46.,
];
pub const GPU_SERIES: &[f32] = &[
    34., 48., 42., 67., 78., 55., 39., 65., 81., 62., 47., 56., 74., 41., 69., 84., 53., 38., 61.,
    76., 44., 58., 73., 42., 57., 69., 51., 42.,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_and_sort_work() {
        let rows = filtered_sorted("code", SortBy::Cpu);
        assert_eq!(rows.len(), 1);
        let all = filtered_sorted("edge", SortBy::Cpu);
        assert_eq!(all.len(), 1);
        let all = filtered_sorted("", SortBy::Cpu);
        assert!(all.windows(2).all(|pair| pair[0].cpu >= pair[1].cpu));
    }


    #[test]
    fn tree_places_children_after_their_parent() {
        let rows = filtered_sorted("", SortBy::Cpu);
        let tree = tree_rows(&rows);
        assert_eq!(tree.len(), rows.len());
        let explorer = tree.iter().position(|(p, _)| p.id == 8844).unwrap();
        let editor = tree.iter().position(|(p, _)| p.id == 10736).unwrap();
        let analyzer = tree.iter().position(|(p, _)| p.id == 13577).unwrap();
        assert!(explorer < editor && editor < analyzer);
        assert_eq!(tree[explorer].1, 0);
        assert_eq!(tree[editor].1, 1);
        assert_eq!(tree[analyzer].1, 2);
    }

    #[test]
    fn filtered_child_becomes_root_without_its_parent() {
        let rows = filtered_sorted("rust-analyzer", SortBy::Name);
        let tree = tree_rows(&rows);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].0.id, 13577);
        assert_eq!(tree[0].1, 0);
    }

    #[test]
    fn tree_recovers_from_cycles_and_empty_input() {
        assert!(tree_rows(&[]).is_empty());
        let mut rows = filtered_sorted("", SortBy::Cpu);
        let left = rows.iter().position(|p| p.id == 8844).unwrap();
        let right = rows.iter().position(|p| p.id == 10736).unwrap();
        rows[left].parent_pid = Some(10736);
        rows[right].parent_pid = Some(8844);
        let tree = tree_rows(&rows);
        assert_eq!(tree.len(), rows.len());
        let ids: std::collections::HashSet<_> = tree.iter().map(|(p, _)| p.id).collect();
        assert_eq!(ids.len(), rows.len());
    }
}
