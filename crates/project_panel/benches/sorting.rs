use criterion::{Criterion, criterion_group, criterion_main};
use fs::MTime;
use project::{Entry, EntryKind, GitEntry, ProjectEntryId};
use project_panel::{par_sort_worktree_entries, par_sort_worktree_entries_with_mode};
use settings::{ProjectPanelSortBy, ProjectPanelSortDirection, ProjectPanelSortMode};
use std::sync::Arc;
use util::rel_path::RelPath;

fn load_linux_repo_snapshot() -> Vec<GitEntry> {
    let file = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/benches/linux_repo_snapshot.txt"
    ))
    .expect("Failed to read file");
    file.lines()
        .filter_map(|line| {
            let kind = match line.chars().next() {
                Some('f') => EntryKind::File,
                Some('d') => EntryKind::Dir,
                _ => return None,
            };

            let entry = Entry {
                kind,
                path: Arc::from(RelPath::unix(&(line.trim_end()[2..])).unwrap()),
                id: ProjectEntryId::default(),
                size: 0,
                inode: 0,
                mtime: None,
                canonical_path: None,
                is_ignored: false,
                is_always_included: false,
                is_external: false,
                is_private: false,
                is_hidden: false,
                char_bag: Default::default(),
                is_fifo: false,
            };
            Some(GitEntry {
                entry,
                git_summary: Default::default(),
            })
        })
        .collect()
}

fn with_assigned_mtimes(snapshot: &[GitEntry], include_missing: bool) -> Vec<GitEntry> {
    snapshot
        .iter()
        .enumerate()
        .map(|(ix, entry)| {
            let mut entry = entry.clone();
            entry.entry.mtime = if include_missing && ix % 4 == 0 {
                None
            } else {
                Some(MTime::from_seconds_and_nanos((ix as u64) + 1, 0))
            };
            entry
        })
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let snapshot = load_linux_repo_snapshot();
    let snapshot_with_mtimes = with_assigned_mtimes(&snapshot, false);
    let snapshot_with_mixed_mtimes = with_assigned_mtimes(&snapshot, true);

    c.bench_function("Sort linux worktree snapshot", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(
                    &mut snapshot,
                    ProjectPanelSortMode::DirectoriesFirst,
                )
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("Sort linux worktree snapshot (Mixed)", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(&mut snapshot, ProjectPanelSortMode::Mixed)
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("Sort linux worktree snapshot (FilesFirst)", |b| {
        b.iter_batched(
            || snapshot.clone(),
            |mut snapshot| {
                par_sort_worktree_entries_with_mode(&mut snapshot, ProjectPanelSortMode::FilesFirst)
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function("Sort linux worktree snapshot (ModifiedTime Descending)", |b| {
        b.iter_batched(
            || snapshot_with_mtimes.clone(),
            |mut snapshot| {
                par_sort_worktree_entries(
                    &mut snapshot,
                    ProjectPanelSortMode::DirectoriesFirst,
                    ProjectPanelSortBy::ModifiedTime,
                    ProjectPanelSortDirection::Descending,
                )
            },
            criterion::BatchSize::LargeInput,
        );
    });

    c.bench_function(
        "Sort linux worktree snapshot (ModifiedTime Descending, Mixed Missing)",
        |b| {
            b.iter_batched(
                || snapshot_with_mixed_mtimes.clone(),
                |mut snapshot| {
                    par_sort_worktree_entries(
                        &mut snapshot,
                        ProjectPanelSortMode::DirectoriesFirst,
                        ProjectPanelSortBy::ModifiedTime,
                        ProjectPanelSortDirection::Descending,
                    )
                },
                criterion::BatchSize::LargeInput,
            );
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
