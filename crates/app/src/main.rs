//! Arke: a file explorer for Windows that never hangs.
//!
//! Usage:
//!   arke [path]
//!   arke --new-window [path]    a window of its own even when Settings › "Single window" is on
//!   arke <path> --bench startup|scroll|scroll-grid [--bench-out file]
//!   arke --bench index [--bench-out file]   build the search index headless, report time and
//!                                           memory (see `bench_index.rs`)
//!   arke --no-resident [path]   don't use or become the resident process
//!   arke --quit                 quit the resident process
//!                               like starting Arke (Settings › "New windows open to")
//!   arke --restore-explorer     undo the folder handler registration and turn the setting off
//!   arke --uninstall [--keep-settings] [--quiet] [--dry-run]
//!                               remove Arke from this PC (see `uninstall.rs`)
//!   arke --shell-host           (internal) hidden context menu process, see `shell_host.rs`
//!   arke --thumb-host           (internal) hidden video thumbnail process, see `thumb_host.rs`
//!   arke --pick-file            (internal) one file dialog for Settings, see `win_picker.rs`
//!
//! By default the first Arke stays in the background after its last window closes, so later
//! launches only open a window without reinitializing the GPU. See `instance.rs`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Tests may unwrap and panic.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::unwrap_in_result,
        clippy::panic,
        reason = "a failing test should stop right there"
    )
)]

// First, so the `tn!` macro is visible in every module below.
#[macro_use]
mod plural;

mod a11y;
mod assets;
mod bench;
mod bench_index;
mod brand;
mod build_info;
mod caption;
mod cloud_places;
mod commands;
mod crashlog;
mod explorer;
mod folder_handler;
mod folder_views;
mod frame_probe;
mod global_search;
mod home;
mod instance;
mod media;
mod motion;
mod net_dialog;
mod op_center;
mod op_queue;
mod palette;
mod place_names;
mod popup;
mod prefs;
mod press_guard;
mod properties;
mod quicklook;
mod recycle_bin;
mod remote_shell;
mod resident;
mod saved_workspaces;
mod scrollbar;
mod search_history;
mod session;
mod settings;
mod shelf;
mod shell;
mod shell_host;
mod slider;
mod spring;
mod teracopy;
mod text_input;
mod theme;
mod thumb_host;
mod tray;
mod uninstall;
mod update;
mod update_quiet;
mod whats_new;
mod win;
mod win_cloud;
mod win_devnotify;
mod win_dragimage;
mod win_eject;
mod win_fast_recycle;
mod win_fileops;
mod win_http;
mod win_icons;
mod win_inuse;
mod win_jumplist;
mod win_menu;
mod win_menu_parts;
mod win_net;
mod win_openinfolder;
mod win_pdf;
mod win_picker;
mod win_popup;
mod win_power;
mod win_purge;
mod win_recycle;
mod win_registry;
mod win_takeover;
mod win_viewsite;
mod win_virtual_drop;
mod workspace;

#[cfg(test)]
mod tests;

rust_i18n::i18n!("locales", fallback = "en");

use std::path::{Path, PathBuf};
use std::sync::Arc;

use gpui::{
};
use gpui_platform::application;

use crate::assets::Assets;
use crate::explorer::Services;
use crate::win::WinShell;
use crate::workspace::Workspace;
use arke_core::takeover::Target;

/// What `--bench` asked for.
enum BenchArg {
    /// `--bench index`: headless index benchmark, no window (`bench_index.rs`)
    Index,
}

#[allow(
    clippy::struct_excessive_bools,
)]
struct Args {
    resident: bool,
    quit: bool,
    /// Stay in the background without opening a window (used for autostart at Windows sign-in)
    background: bool,
    /// Undo the folder handler registration, then exit
    restore_explorer: bool,
    /// A window of its own, not a tab in the window used last (`--new-window`)
    new_window: bool,
}

fn parse_args() -> Args {
    parse_args_from(std::env::args_os().skip(1))
}

    let mut args = Args {
        path: None,
        bench: None,
        bench_out: None,
        resident: true,
        quit: false,
        background: false,
        page: None,
        restore_explorer: false,
        new_window: false,
    };
    while let Some(arg) = it.next() {
        match arg.to_str() {
            Some("--bench") => {
                args.bench = Some(match it.next().as_ref().and_then(|s| s.to_str()) {
                    Some("index") => BenchArg::Index,
                });
            }
            Some("--bench-out") => args.bench_out = it.next().map(PathBuf::from),
            Some("--no-resident") => args.resident = false,
            Some("--quit") => args.quit = true,
            Some("--background") => args.background = true,
            // like Arke itself, last tabs included (#73). The name stays: it is in the registry.
            Some("--home") => {}
            Some("--restore-explorer") => args.restore_explorer = true,
            // Just updated: the old process's pid follows (already waited for at the start of
            // app_main).
                let _ = it.next();
                update::mark_just_updated();
            }
            // An empty argument (`arke.exe ""`, a shortcut with an empty %1) names no folder.
            _ if arg.is_empty() => {}
            _ => args.path = Some(path_arg(&arg)),
        }
    }
    // Updated in the background with no window open: the next window opens where the settings
    // say, as after any background start; there were no tabs to bring back.
    if args.background {
        update::start_handled();
    }
    args
}

/// A folder argument as the shell passes it. The registered command is `"arke.exe" "%1"`, and for a
/// drive root `%1` ends in a backslash (`"C:\"`), which the command line rules read as an escaped
/// quote: the argument arrives as `C:"`. The quote is the leftover of that, so it goes and the
/// backslash comes back.
    if units.last() == Some(&u16::from(b'"')) {
        units.pop();
        if units.last() != Some(&u16::from(b'\\')) {
            units.push(u16::from(b'\\'));
        }
    }
}

impl Args {
    /// The window benchmark to run, if any.
        match self.bench {
            Some(BenchArg::Window(mode)) => Some(mode),
            _ => None,
        }
    }
}

fn unix_ms_now() -> u64 {
    SystemTime::now()
        .map_or(0, |d| d.as_millis() as u64)
}

/// Convert another process's launch time to an `Instant` here, so measurements start at launch.
fn instant_from_unix_ms(launched_unix_ms: u64) -> Instant {
    let now = Instant::now();
    now.checked_sub(ago).unwrap_or(now)
}

/// Single binary: `arke.exe --indexd <command>` is the background service (arke-indexd) and opens
/// no window.

    }
        rust_i18n::set_locale(&win::ui_locale());
        return uninstall::run_cli(&args[2..]);
    }
    // A Browse button's file dialog, shown in its own process (see `win_picker.rs`).
        win_picker::run();
    }
    let mut args = parse_args();
    // As the folder handler the shell hands over every folder, virtual ones included: what Arke
    if let Some(target) = args.path.as_deref().and_then(Path::to_str) {
        match arke_core::takeover::route_argument(target) {
                win::open_in_explorer(&target);
            }
            // open Arke too.
                win::open_in_explorer(target);
            }
            // does, where Settings › "New windows open to" says (last tabs included) (#73).
                args.path = None;
            }
                args.path = None;
            }
        }
    }
    // Nothing else starts: no log file, tray, window or resident process.
    if args.restore_explorer {
        return if folder_handler::restore_cli() {
        } else {
        };
    }
    // Before anything in app_main: no log file, residency handoff, tray or window.
    if args.bench == Some(BenchArg::Index) {
        return bench_index::run(args.bench_out.as_deref());
    }
    app_main(args);
}

#[allow(
    clippy::too_many_lines,
    reason = "Startup order (args, handoff to the resident process, window, settings-driven pieces) reads best top to bottom in one place"
)]
fn app_main(mut args: Args) {
    if run_shell_host() || run_thumb_host() {
        return;
    }
    // Launched by the old version (auto-update): wait for it to exit before taking over residency.
    let started_at = Instant::now();
    // A launch in the background opens no window; any it opens later is no cold start.
    if !args.background {
        explorer::mark_process_start(started_at);
    }
    let launched_unix_ms = unix_ms_now();
    crashlog::init();
    motion::refresh_system_setting();
    rust_i18n::set_locale(&win::ui_locale());
    theme::set_mica_available(use_mica());
    // From here any filesystem call on this thread trips a debug assertion.
    arke_core::fs::mark_ui_thread();

    // handing off. std::path::absolute is string handling only (GetFullPathNameW) and never touches
    // the filesystem.
    if let Some(path) = &args.path
    {
        args.path = Some(abs);
    }

    // `arke --quit` (the installer before it replaces the exe): returns once the resident Arke is
    // gone, so its files are free.
    if args.quit {
            return;
        };
            instance::Quit::Gone => log::info!("--quit: resident Arke gone"),
                "--quit: the resident Arke is another copy ({}), left running",
                other.display()
            ),
        }
        return;
    }
    let linger = shell_window(&args);
    let listener = if args.background {
        // Background launch: if a resident process exists we're not needed; otherwise become
        // resident without a window.
        match instance::listen() {
            Some(listener) => Some(listener),
            None => return,
        }
    } else if args.resident || args.quit {
        let request = if args.quit {
        } else {
                path: args.path.clone(),
                bench: args.window_bench(),
                bench_out: args.bench_out.clone(),
                launched_unix_ms,
                page: args.page,
                new_window: args.new_window,
            })
        };
        match hand_off(&request) {
                // A "Show in folder" caller may still be selecting: stay until the stand-in
                // shell window is done.
                if let Some(linger) = linger {
                    linger.wait();
                }
                return;
            }
        }
    } else {
        None
    };
    if args.quit {
        log::info!("no resident Arke running");
        return;
    }

    let background = args.background;
    let bench = args
        .window_bench()
        .map(|mode| Bench::new(mode, args.bench_out));
    let start_path = args.path;
    let start_page = args.page;
    let resident = listener.is_some();
    log::info!(
        "{}",
        if resident {
            "resident mode"
        } else {
            "standalone mode"
        }
    );

    let app = application().with_assets(Assets);
    // gpui on Windows quits when the last window closes; resident mode decides when to quit itself.
    let app = if resident {
    } else {
        app
    };
    app.run(move |cx: &mut App| {
        start_type_names(cx);
        motion::sync_app(cx);
        frame_probe::init();
        // Settings load in the background; benchmarks neither read nor write, so test folders don't
        let store = settings::SettingsStore::init(
            if bench.is_some() {
                None
            } else {
                settings::default_path()
            },
            cx,
        );
        // Keys, with the shortcuts changed in Settings
        commands::init(&store, cx);
        // Per-folder views, next to the settings file; benchmarks keep them in memory only
            if bench.is_some() {
                None
            } else {
                folder_views::default_path()
            },
            cx,
        );
        // What the user opens, for ranking "This PC" results; benchmarks keep it in memory only
        search_history::SearchHistory::init(
            if bench.is_some() {
                None
            } else {
                search_history::default_path()
            },
            cx,
        );
        // Saved workspaces, next to the settings file too
        saved_workspaces::WorkspaceStore::init(
            if bench.is_some() {
                None
            } else {
                saved_workspaces::default_path()
            },
            cx,
        );
        // The shelf, next to the settings file; benchmarks keep it in memory only
        shelf::ShelfStore::init(
            if bench.is_some() {
                None
            } else {
                shelf::default_path()
            },
            cx,
        );
        follow_language_setting(&store, cx);
        let remotes = remote_shell::start(&store, cx);
        let services = Services {
                Arc::clone(&remotes),
            )),
                Arc::new(WinShell),
                Arc::clone(&remotes),
            )),
            remotes,
        };
        if bench.is_none() {
            let shell = Arc::clone(&services.shell);
            explorer::spawn_thread("editor-apps", move || {
                let apps = shell.editor_apps();
                // Icons come from each exe (icon only, no thumbnail provider).
                let icons = apps
                    .iter()
                    .map(|(_, exe)| shell.file_icon(exe, 32))
                    .collect();
                explorer::set_open_with(apps, icons);
            });
        }
        if bench.is_none() {
            follow_drive_removals(cx);
            teracopy::detect_in_background();
            quicklook::detect_in_background();
        }
        if resident && bench.is_none() {
            install_tray(cx, &services, &store);
            follow_autostart_setting(&store, &services, cx);
            follow_takeover_setting(&store, &services, cx);
        }
        // The shell registration follows the setting in every process that is running, resident
        // or not (a standalone Arke started through the folder verb keeps it repaired too).
        if bench.is_none()
            && let Some(handler) = folder_handler::Handler::real()
        {
            folder_handler::follow_setting(&store, handler, cx);
        }
        if bench.is_none() {
            follow_jump_list(&store, cx);
            follow_start_menu_setting(&store, &services, cx);
        }
        // Global search: index only when enabled; not during benchmarks.
        if bench.is_none() {
            global_search::GlobalSearch::init(
                Arc::clone(&services.fs),
                Arc::clone(&services.shell),
                global_search::default_cache_path(),
                cx,
            );
        }
        // Auto-update: delete the old binary left from last time; check a bit after startup so it
        // doesn't compete, then every hour while Arke keeps running (it can stay in the
        // notification area for days). `auto_check` only checks when 20 hours have passed.
        // A download that is waiting (from before Windows restarted) waits again; with
        // automatic updates on it goes in when Arke quits (`update_quiet.rs`).
        if bench.is_none() {
            let (pending_tx, pending_rx) = flume::bounded(1);
            explorer::spawn_thread("update-cleanup", move || {
                let _ = pending_tx.send(update::clean_up_old());
            });
            update_quiet::mark_resident(resident, background);
            cx.on_app_quit(|cx| {
                update_quiet::on_quit(cx);
                async {}
            })
            .detach();
            if !resident {
                // gpui quits by itself when the last window closes here.
                cx.on_window_closed(|cx, _| {
                    if cx.windows().is_empty() {
                    }
                })
                .detach();
            }
            cx.spawn(async move |cx| {
                cx.background_executor()
                    .await;
                    cx.update(|cx| update_quiet::restore(file, cx));
                }
                loop {
                    cx.update(update::auto_check);
                    cx.background_executor()
                        .await;
                }
            })
            .detach();
        }
        let benchmarking = bench.is_some();
        let first_window_opened = if background {
            true
        } else if let Some(page) = start_page {
            open_target(cx, &services, page.target(), &[], None)
        } else {
            open_explorer(cx, &services, start_path, bench, started_at)
        };
        if !first_window_opened && !resident {
            cx.quit();
            return;
        }
        if let Some(listener) = listener {
            start_resident_service(cx, listener, services);
            quit_with_last_window_unless_resident(cx);
            resident::watch(cx);
            if background {
                resident::enter(cx);
            }
        }
        if !benchmarking {
            prewarm_shell_host();
        }
        if !background {
            cx.activate(true);
        }
    });
}

/// on the notification thread (`win_devnotify.rs`); here every window moves its tabs off the drive.
fn follow_drive_removals(cx: &mut App) {
    let events = win_devnotify::start();
    cx.spawn(async move |cx| {
            cx.update(|cx| workspace::leave_drive_everywhere(&drive, cx));
        }
    })
    .detach();
}

/// "Start at Windows sign-in": when on, put a shortcut in the Startup folder pointing at the
/// background, no registry.
fn follow_autostart_setting(
    services: &Services,
    cx: &mut App,
) {
    let shell = Arc::clone(&services.shell);
    cx.observe(store, move |store, cx| {
        let store = store.read(cx);
        // Before settings load the values are defaults; don't delete the shortcut based on them.
        if !store.loaded {
            return;
        }
        let wanted = store.settings.autostart;
        if applied == Some(wanted) {
            return;
        }
        // First time seeing "off" and never on before: no need to touch the filesystem.
        let first = applied.is_none();
        applied = Some(wanted);
        if first && !wanted {
            return;
        }
        let shell = Arc::clone(&shell);
        explorer::spawn_thread("autostart", move || {
                log::warn!("autostart shortcut not updated: {err}");
            }
        });
    })
    .detach();
}

/// "Add to the Start menu": when on, write the Start menu shortcut at every start (so it follows a
/// moved exe) and whenever it is turned on; when turned off, delete it. Background thread, no
/// registry.
fn follow_start_menu_setting(
    services: &Services,
    cx: &mut App,
) {
    // The installer made its own shortcut and removes it with Arke; the setting is hidden then.
    if update::installed_by_setup() {
        return;
    }
    let shell = Arc::clone(&services.shell);
    let mut apply = move |store: &settings::SettingsStore| {
        if !store.loaded {
            return;
        }
        let wanted = store.settings.start_menu;
        if applied == Some(wanted) {
            return;
        }
        let first = applied.is_none();
        applied = Some(wanted);
        if first && !wanted {
            return;
        }
        let shell = Arc::clone(&shell);
        explorer::spawn_thread("start-menu", move || {
                log::warn!("Start menu shortcut not updated: {err}");
            }
        });
    };
    apply(store.read(cx));
    cx.observe(store, move |store, cx| apply(store.read(cx)))
        .detach();
}

/// setting off stops the watcher at once.
fn follow_takeover_setting(
    services: &Services,
    cx: &mut App,
) {
    let services = services.clone();
    cx.spawn(async move |cx| {
            let opened = cx.update(|cx| open_taken_over(cx, &services, &request));
            let _ = request.reply.send(opened);
        }
    })
    .detach();
    let apply = move |store: &settings::SettingsStore| {
        // Before settings load the values are defaults.
        if !store.loaded {
            return;
        }
        let wanted = store.settings.open_folders_in_arke && store.settings.keep_running;
        tray::set_takeover(wanted);
        let mut watcher = watcher.borrow_mut();
        if wanted && watcher.is_none() {
            log::info!("takeover: starting");
            *watcher = win_takeover::Watcher::start(tx.clone());
        } else if !wanted {
            *watcher = None;
        }
    };
    // Settings may already be loaded; the observer only hears later changes.
    apply(store.read(cx));
    cx.observe(store, move |store, cx| apply(store.read(cx)))
        .detach();
}

/// again every half minute (one registry read, off the UI thread).
) {
    if !forced && !folder_handler::shell_allowed() {
        return;
    }
    let (tx, rx) = flume::bounded::<()>(4);
    let services = services.clone();
    cx.spawn(async move |cx| {
        while rx.recv_async().await.is_ok() {
            // Like starting Arke: where Settings › "New windows open to" says, last tabs
            // included (#73); in single window mode, a tab there.
            cx.update(|cx| open_fresh(cx, &services, Instant::now()));
        }
    })
    .detach();
    let apply = {
        move |store: &settings::SettingsStore| {
            // Before settings load the values are defaults.
            if !store.loaded {
                return;
            }
            let s = &store.settings;
            let wanted = s.open_folders_in_arke
                && s.win_e_opens_arke
                && s.keep_running
                && direct_launch.get();
            let mut hook = hook.borrow_mut();
            if wanted && hook.is_none() {
            } else if !wanted && hook.is_some() {
                *hook = None;
            }
        }
    };
    {
        cx.observe(store, move |store, cx| apply(store.read(cx)))
            .detach();
    }
    let store = store.downgrade();
    cx.spawn(async move |cx| {
        loop {
            let active = cx
                .background_executor()
        .spawn(async move { forced })
                .await;
            if direct_launch.replace(active) != active {
                log::info!(
                    if active { "on" } else { "off" }
                );
            }
            let Some(store) = store.upgrade() else {
                return;
            };
            cx.update(|cx| apply(store.read(cx)));
            cx.background_executor()
                .await;
        }
    })
    .detach();
}

    open_target(
        cx,
        services,
        request.target.clone(),
        &request.select,
        Some(request.explorer_thread),
    )
}

/// single window mode as a tab in the window used last (`tab_host`), otherwise in a new window.
fn open_target(
    cx: &mut App,
    services: &Services,
    target: Target,
    select: &[String],
) -> bool {
    open_target_timed(cx, services, target, select, explorer_thread, None)
}

/// [`open_target`]; a window it opens for a request that arrived at `received` is timed from then
/// in the log.
fn open_target_timed(
    cx: &mut App,
    services: &Services,
    target: Target,
    select: &[String],
) -> bool {
    if let Some(host) = tab_host(cx)
        && show_in_tab(cx, host, target.clone(), select.clone(), explorer_thread)
    {
        return true;
    }
    // says (the last session, say).
    let asked =
        (target != Target::Home || last_window(cx).is_some()).then_some(Asked { target, select });
    let opened = open_explorer_window(cx, services, asked, None, Instant::now(), received);
    let Some(window) = opened else {
        return false;
    };
    let shown = window.update(cx, |_, window, _| {
        bring_to_front(window, explorer_thread);
    });
    if shown.is_ok() {
        cx.activate(true);
    }
    shown.is_ok()
}

/// one used last, else the newest. `None` (open a window) when no window is open or single window
/// mode is off.
    if !settings::SettingsStore::global(cx)
        .read(cx)
        .settings
        .single_window
    {
        return None;
    }
    last_window(cx)
}

/// Shows `target` as a tab in `host` and brings it to the front; `false` if the window is gone.
fn show_in_tab(
    cx: &mut App,
    host: gpui::WindowHandle<Workspace>,
    target: Target,
) -> bool {
    let shown = host.update(cx, |workspace, window, cx| {
        workspace.open_external(target, select, window, cx);
        bring_to_front(window, explorer_thread);
    });
    if shown.is_ok() {
        cx.activate(true);
    }
    shown.is_ok()
}

/// the window used last, on what "New windows open to" says ([`fresh_tab`]); otherwise a window
/// started like Arke itself.
fn open_fresh(cx: &mut App, services: &Services, received: Instant) -> bool {
    if let Some(host) = tab_host(cx) {
        let target = {
            let s = &settings::SettingsStore::global(cx).read(cx).settings;
            fresh_tab(s.start_location, s.recent.first().cloned(), home_folder())
        };
            return true;
        }
    }
    open_in_front(cx, services, None, None, received, received)
}

/// What a tab opened with no folder shows, after "New windows open to": the last tabs are open
/// already, so that one shows Home, as a second window does (#178).
    use settings::StartLocation;
    match start {
        StartLocation::HomePage | StartLocation::LastSession => Target::Home,
        StartLocation::Home => Target::Folder(home),
        StartLocation::LastFolder => Target::Folder(last.unwrap_or(home)),
    }
}

/// The user profile folder (string handling only).
fn home_folder() -> PathBuf {
}

/// A request from another `arke.exe` (or the shell through it) to open something.
    let started_at = instant_from_unix_ms(open.launched_unix_ms);
    let bench = open.bench.map(|mode| Bench::new(mode, open.bench_out));
    // The sender was just started by the user or the shell and passed on its foreground right
    // (instance.rs), which `bring_to_front` makes use of.
    if let Some(page) = open.page {
        open_target(cx, services, page.target(), &[], None)
    } else if bench.is_some() || open.new_window {
        open_in_front(cx, services, open.path, bench, started_at, received)
    } else if let Some(path) = open.path {
        // A folder handed over by the shell or another program (#11).
        open_target_timed(
            cx,
            services,
            Target::Folder(path),
            &[],
            None,
            Some(received),
        )
    } else {
        open_fresh(cx, services, received)
    }
}

/// How long after a maximized window opened behind the others Windows may still activate it: it

/// last, or the window something was asked for in), each where it was, a turn apart: the request
/// that started the session, a second `arke.exe` waiting for its answer (`instance.rs`), is
/// answered once `front` is up. Then `front` comes forward again, ahead of any of them that
/// Windows activated on showing it.
pub(crate) fn open_session_windows(
    cx: &mut App,
    services: &Services,
    front: gpui::AnyWindowHandle,
) {
    let services = services.clone();
    cx.spawn(async move |cx| {
        // The one used last of these opens last, so after `front` it is the next in line.
        for saved in windows.into_iter().rev() {
            next_turn().await;
            let opened = cx.update(|cx| {
                let path = saved.first_folder().cloned()?;
                let restored = saved.place.and_then(|p| restored_bounds(p, cx));
                let start = WindowStart {
                    path,
                    default: false,
                    session: Some(saved.session),
                    bounds: restored.map(|r| r.bounds),
                    display: restored.and_then(|r| r.display),
                    focus: false,
                };
                open_workspace_window(cx, &services, start, None, Instant::now(), None)
            });
            behind.extend(opened.map(gpui::AnyWindowHandle::from));
        }
        cx.update(|cx| {
            // Unless the user went on to another window meanwhile.
            let last = cx
                .try_global::<workspace::LastActiveWindow>()
                .and_then(|l| l.0);
            if last.is_some_and(|l| l != front && !behind.contains(&l)) {
                return;
            }
            if front
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
            {
                // First in the session even if Windows doesn't let it take the foreground.
                session::activated(front, cx);
            }
        });
    })
    .detach();
}

/// Lets the UI thread get to other work (an answer to send, a frame to draw) before going on.
async fn next_turn() {
    let mut yielded = false;
    std::future::poll_fn(|task| {
        if yielded {
        } else {
            yielded = true;
            task.waker().wake_by_ref();
            std::task::Poll::Pending
        }
    })
    .await;
}

/// The Arke window used last, else the newest; `None` when no window is open.
        .windows()
        .into_iter()
        .filter_map(|w| w.downcast::<Workspace>())
        .collect();
    let last = cx
        .try_global::<workspace::LastActiveWindow>()
        .and_then(|l| l.0)
        .and_then(|w| w.downcast::<Workspace>());
    pick_tab_host(&windows, last)
}

/// `last` if it is still open, otherwise the newest window.
    last.filter(|l| windows.contains(l))
        .or_else(|| windows.last().copied())
}

/// Brings Arke's window to the front, even when another program owns the foreground (a window
        win::bring_to_front(hwnd, explorer_thread.filter(|&t| t != 0));
    }
}

/// "Show in folder" while Arke is the folder handler: a stand-in shell window for the folder that
/// collects what the caller selects (`win_openinfolder.rs`) and forwards it to the running Arke.
    let folder = args.path.as_ref().filter(|p| p.is_absolute())?;
    if args.background
        || args.quit
        || args.new_window
        || args.page.is_some()
        || args.bench.is_some()
    {
        return None;
    }
    // What the shell or the calling program passed, to tell "Show in folder" callers apart.
    log::info!(
        "opened with {:?}",
    );
    win_openinfolder::start(folder.clone(), |folder, items| {
        let count = items.len();
        log::info!("shell window: {count} selected items forwarded: {outcome:?}");
    })
}

/// Selects `items` in the newest window that shows (or is loading) their folder, or opens one.
fn select_items(cx: &mut App, services: &Services, folder: &Path, items: &[PathBuf]) -> bool {
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let names = arke_core::takeover::selected_names(folder, &full);
    if names.is_empty() {
        return false;
    }
    for handle in cx.windows().into_iter().rev() {
        let Some(workspace) = handle.downcast::<Workspace>() else {
            continue;
        };
        let done = workspace.update(cx, |workspace, _, cx| {
            workspace.active_view().update(cx, |view, cx| {
                view.select_in_folder(folder, shown.clone(), cx)
            })
        });
            log::info!(
                "selected {} items in the window showing {}",
                shown.len(),
                folder.display()
            );
            return true;
        }
    }
    open_target(
        cx,
        services,
        Target::Folder(folder.to_path_buf()),
        &names,
        None,
    )
}

/// only the latest request, so quick folder changes don't queue.
    let (tx, rx) = flume::unbounded::<Lists>();
    explorer::spawn_thread("jump-list", move || {
                lists = newer;
            }
                log::info!("jump list not updated: {err}");
            }
        }
    });
    cx.observe(store, move |store, cx| {
        let store = store.read(cx);
        if !store.loaded {
            return;
        }
        let s = &store.settings;
            .favorites
            .iter()
            .flatten()
            .chain(&s.pinned)
            .cloned()
            .collect();
        let lists = (pinned, recent);
        if applied.as_ref() == Some(&lists) {
            return;
        }
        applied = Some(lists.clone());
        let _ = tx.send(lists);
    })
    .detach();
}

/// "Language" setting: switch when loaded or changed and redraw all windows (input placeholders
/// update in new tabs only).
    cx.observe(store, move |store, cx| {
        let language = store.read(cx).settings.language;
        if applied == Some(language) {
            return;
        }
        applied = Some(language);
        apply_language(&language.locale(), cx);
    })
    .detach();
    // Also apply what is set now: the observer only hears later changes.
    apply_language(&store.read(cx).settings.language.locale(), cx);
}

/// Starts looking up Windows' names for file types the built-in table doesn't have; windows
/// repaint (and re-sort by type) when names arrive.
fn start_type_names(cx: &mut App) {
    let (tx, rx) = flume::bounded::<()>(1);
    let notify = tx.clone();
    arke_core::format::start_type_names(win_dragimage::registered_type_name, move || {
        let _ = tx.try_send(());
    });
    media::start_icon_probe(win_dragimage::registered_default_icon, move || {
        let _ = notify.try_send(());
    });
    cx.spawn(async move |cx| {
        while rx.recv_async().await.is_ok() {
            cx.update(App::refresh_windows);
        }
    })
    .detach();
}

/// Switches strings and the CJK font fallback order to `locale`, repainting if either changed.
fn apply_language(locale: &str, cx: &mut App) {
    // Fonts follow the UI language so Simplified text gets Simplified glyphs on a Traditional
    // system and vice versa (see `theme::cjk_fallbacks`).
    let mut changed = theme::set_ui_language(locale, win::system_locale());
    if *rust_i18n::locale() != *locale {
        rust_i18n::set_locale(locale);
        arke_core::format::language_changed();
        changed = true;
    }
    if changed {
        cx.refresh_windows();
    }
}

/// In resident mode with "Keep running in background" off: quit when the last window closes.
fn quit_with_last_window_unless_resident(cx: &mut App) {
    cx.on_window_closed(|cx, _| {
        let keep = settings::SettingsStore::global(cx)
            .read(cx)
            .settings
            .keep_running;
        if !keep && cx.windows().is_empty() {
            log::info!("background residency off: last window closed, quitting");
        }
    })
    .detach();
}

fn use_mica() -> bool {
    win::supports_mica()
}

/// New window size: 75% of the usable screen (so large screens don't get a tiny window), clamped to
/// 1180×760 .. 2200×1400.
fn default_window_size(cx: &App) -> gpui::Size<gpui::Pixels> {
    let (min, max) = (size(px(1180.), px(760.)), size(px(2200.), px(1400.)));
    let Some(screen) = cx.primary_display().map(|d| d.visible_bounds().size) else {
        return min;
    };
    let fit = |want: gpui::Pixels, lo: gpui::Pixels, hi: gpui::Pixels, room: gpui::Pixels| {
        // Screens smaller than the minimum get 95% of the screen.
        (want.max(lo).min(hi)).min(room * 0.95)
    };
    size(
        fit(screen.width * 0.75, min.width, max.width, screen.width),
        fit(screen.height * 0.75, min.height, max.height, screen.height),
    )
}

/// Where a saved window opens again.
    bounds: WindowBounds,
    /// The screen it goes on; `None` for the main one. Windows checks the bounds against this
    /// screen, and would put a window meant for another one at the main screen's default spot.
}

/// Last window position, on the screen that shows most of it.
    let main = cx.primary_display().map(|d| d.bounds());
    place_on_screens(place, &screens, main)
}

/// [`restored_bounds`]: the screen showing most of the window, if at least a 240×120 patch of it
/// is still on one (so it never opens off-screen after unplugging a monitor). A maximized window
/// whose screen is gone is maximized on the main screen (`main`) instead; a window that wasn't
/// gets the default spot (`None`).
fn place_on_screens(
    place: settings::WindowPlace,
    let bounds = Bounds::new(
        gpui::point(px(place.x), px(place.y)),
        size(px(place.width.max(720.)), px(place.height.max(480.))),
    );
    let shown = |screen: &Bounds<gpui::Pixels>| {
        let overlap = screen.intersect(&bounds);
        (overlap.size.width >= px(240.) && overlap.size.height >= px(120.))
            .then(|| f32::from(overlap.size.width) * f32::from(overlap.size.height))
    };
    let screen = screens
        .iter()
        .filter_map(|(id, screen)| Some((*id, shown(screen)?)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(id, _)| id);
    match screen {
            bounds: if place.maximized {
            } else {
                WindowBounds::Windowed(bounds)
            },
            display: Some(id),
        }),
        None if place.maximized => main.map(|main| {
            let size = bounds.size.min(&main.size);
            let origin = gpui::point(
                main.origin.x + (main.size.width - size.width) / 2.,
                main.origin.y + (main.size.height - size.height) / 2.,
            );
                display: None,
            }
        }),
        None => None,
    }
}

/// Settings read only after the first window opened (a slow disk): it moves to where the window
/// it takes the tabs of was.
pub(crate) fn move_to_place(window: &mut gpui::Window, place: settings::WindowPlace, cx: &App) {
    let Some(restored) = restored_bounds(place, cx) else {
        return;
    };
    let bounds = restored.bounds.get_bounds();
    // gpui can resize a window but not move it.
        let screen = restored
            .display
            .and_then(|id| cx.find_display(id))
            .or_else(|| cx.primary_display());
        if let Some(screen) = screen {
            win::move_window(hwnd, u64::from(screen.id()), screen.bounds(), bounds);
        }
    }
        window.zoom_window();
    }
}

/// `arke.exe --thumb-host`: the video thumbnail process (see `thumb_host.rs`). No window, no
/// residency or settings.
fn run_thumb_host() -> bool {
    if std::env::args_os()
        .nth(1)
    {
        return false;
    }
    thumb_host::run();
    true
}

/// `arke.exe --shell-host`: the menu process. No window, no residency or settings; only serves menu
/// requests from Arke.
fn run_shell_host() -> bool {
    if std::env::args_os()
        .nth(1)
    {
        return false;
    }
    rust_i18n::set_locale(&win::ui_locale());
    shell_host::run();
    true
}

/// doesn't wait.
fn prewarm_shell_host() {
    let _ = std::thread::Builder::new()
        .name("shell-host-prewarm".into())
        .spawn(|| {
            shell_host::prewarm();
        });
}

    /// No resident process; this one becomes resident
    /// without residency
    Standalone,
}

    for _ in 0..2 {
                log::warn!("resident process did not respond in time, running standalone");
            }
                }
                if let Some(listener) = instance::listen() {
                }
                // Two Arkes launched together and the other became resident first: retry the
                // handoff once.
            }
        }
    }
}

fn start_resident_service(cx: &mut App, listener: instance::Listener, services: Services) {
    instance::serve(listener, move |request| {
        let received = Instant::now();
        let (ack_tx, ack_rx) = flume::bounded(1);
        if tx.send((request, received, ack_tx)).is_err() {
            return false;
        }
        // If the UI thread doesn't handle it in time, reply failure so the sender opens its own
        // window.
        ack_rx
            .unwrap_or(false)
    });
    cx.spawn(async move |cx| {
            let handled = cx.update(|cx| match request {
                    true
                }
            });
            let _ = ack.send(handled);
        }
    })
    .detach();
}

/// tests and screenshots).
        return;
    }
    let Some(commands) = tray::install() else {
        return;
    };
    // The setting can turn it off: add or remove when settings load or change. This runs on every
    // settings save (folder changes included), so do nothing and log nothing if the state is
    // unchanged.
    let last = std::cell::Cell::new(None);
    let show = move |on: bool| {
        if last.replace(Some(on)) == Some(on) {
            return;
        }
        if !tray::set_visible(on) {
            log::warn!("cannot add notification area icon");
        } else if on {
            log::info!("notification area icon added");
        }
    };
    show(store.read(cx).settings.tray_icon);
    cx.observe(store, move |store, cx| {
        show(store.read(cx).settings.tray_icon);
    })
    .detach();
    let services = services.clone();
    cx.spawn(async move |cx| {
            cx.update(|cx| match command {
                    // Brings back the window used last (restored if minimized); a new window only
                    // when none is open, so repeated or double clicks don't pile up windows.
                    if let Some(window) = last_window(cx) {
                        let _ = window.update(cx, |_, window, _| bring_to_front(window, None));
                        cx.activate(true);
                    } else if open_explorer(cx, &services, None, None, Instant::now()) {
                        cx.activate(true);
                    }
                }
                tray::TrayCommand::ToggleTakeover => {
                    settings::SettingsStore::global(cx).update(cx, |store, cx| {
                        store.update(
                            |st| {
                                st.open_folders_in_arke = !st.open_folders_in_arke;
                                if st.open_folders_in_arke {
                                    st.keep_running = true;
                                }
                            },
                            cx,
                        );
                    });
                }
                tray::TrayCommand::Quit => {
                    tray::remove();
                }
                tray::TrayCommand::Update => {
                    let window = last_window(cx).or_else(|| {
                        open_explorer(cx, &services, None, None, Instant::now())
                            .then(|| last_window(cx))
                            .flatten()
                    });
                    if let (Some(window), update::UpdateState::Available(release)) =
                        (window, update::UpdateState::get(cx))
                    {
                        let _ = window.update(cx, |ws, window, cx| {
                            bring_to_front(window, None);
                            ws.open_whats_new(
                                window,
                                cx,
                            );
                        });
                        cx.activate(true);
                    }
                }
            });
        }
    })
    .detach();
    // Quitting from elsewhere (arke --quit, last window closed) also removes the icon, so no ghost
    // icon lingers until hovered.
    cx.on_app_quit(|_| async { tray::remove() }).detach();
    // The menu offers an update that was found (and not yet installed).
    cx.observe_global::<update::UpdateState>(|cx| {
        tray::set_update(match update::UpdateState::get(cx) {
            update::UpdateState::Available(r) if !update_quiet::hidden(cx) => Some(r.version),
            _ => None,
        });
    })
    .detach();
}

fn open_explorer(
    cx: &mut App,
    services: &Services,
    started_at: Instant,
) -> bool {
    let asked = path.map(Asked::folder);
    open_explorer_window(cx, services, asked, bench, started_at, None).is_some()
}

/// when the request for it arrived, which its start is timed from in the log.
fn open_in_front(
    cx: &mut App,
    services: &Services,
    started_at: Instant,
    received: Instant,
) -> bool {
    let asked = path.map(Asked::folder);
    let Some(handle) = open_explorer_window(cx, services, asked, bench, started_at, Some(received))
    else {
        return false;
    };
    let _ = handle.update(cx, |_, window, _| bring_to_front(window, None));
    cx.activate(true);
    true
}

/// What a window is opened for: a folder (from the command line, another program or a second
struct Asked {
    target: Target,
}

impl Asked {
    const fn folder(path: PathBuf) -> Self {
        Self {
            target: Target::Folder(path),
        }
    }
}

/// with "Last tabs" brings back the last session, also when something was asked for: that opens
/// as a tab of the window used last in single window mode, else in a window of its own in front of
/// the session's.
fn open_explorer_window(
    cx: &mut App,
    services: &Services,
    started_at: Instant,
    // The startup path is string handling only; existence isn't checked, since startup must not
    // touch locations that might block. No path given: if settings say "last location/last tabs",
    // open there (switching once settings finish loading).
    let first_window = cx.windows().is_empty();
    let (last, mut sessions, place, single_window, loaded) = {
        let store = settings::SettingsStore::global(cx).read(cx);
        let start = update::start_location(store.settings.start_location);
        let last =
            (asked.is_none() && store.loaded && start == settings::StartLocation::LastFolder)
                .then(|| store.settings.recent.first().cloned())
                .flatten();
        let sessions = if store.loaded
            && first_window
            && bench.is_none()
            && start == settings::StartLocation::LastSession
        {
            store.settings.last_windows()
        } else {
        };
        // Window position: the first window uses the last one; later windows are centered instead
        // of stacking.
        let place = (first_window && bench.is_none())
            .then_some(store.settings.window)
            .flatten();
        if store.loaded {
            update::start_handled();
        }
        (
            last,
            sessions,
            place,
            store.settings.single_window,
            store.loaded,
        )
    };
    // In single window mode the session comes back as usual, and what was asked for opens as a tab
    // of its front window.
    let (asked, as_tab) = match asked {
        Some(asked) if single_window && !sessions.is_empty() => (None, Some(asked)),
        asked => (asked, None),
    };
    if let Some(asked) = asked {
        let path = match &asked.target {
            Target::Folder(path) => path.clone(),
        };
        let restored = place.and_then(|p| restored_bounds(p, cx));
        let start = WindowStart {
            path,
            default: false,
            session: None,
            bounds: restored.map(|r| r.bounds),
            display: restored.and_then(|r| r.display),
            focus: true,
        };
        let wait_for_session = first_window && !loaded && bench.is_none();
        let handle = open_workspace_window(cx, services, start, bench, started_at, received)?;
        let _ = handle.update(cx, |workspace, window, cx| {
            workspace.show_taken_over(asked.target, asked.select, window, cx);
            // Settings not read yet: the last session may still come back around this window.
            if wait_for_session {
                workspace.restore_session_when_read();
            }
        });
        if !sessions.is_empty() {
            // Single window mode off: the session's windows open behind this one.
            open_session_windows(cx, services, sessions, handle.into());
        }
        return Some(handle);
    }
    // This window takes the one used last; the others open behind it once it is up.
    let front = (!sessions.is_empty()).then(|| sessions.remove(0));
    let start_default = last.is_none() && front.is_none();
    let path = last
        .or_else(|| front.as_ref().and_then(|w| w.first_folder().cloned()))
        .unwrap_or_else(home_folder);
    let place = front.as_ref().and_then(|w| w.place).or(place);
    let restored = place.and_then(|p| restored_bounds(p, cx));
    let start = WindowStart {
        path,
        default: start_default,
        session: front.map(|w| w.session),
        bounds: restored.map(|r| r.bounds),
        display: restored.and_then(|r| r.display),
        focus: true,
    };
    let handle = open_workspace_window(cx, services, start, bench, started_at, received)?;
    if let Some(asked) = as_tab {
        let _ = handle.update(cx, |workspace, window, cx| {
            workspace.open_external(asked.target, asked.select, window, cx);
        });
    }
    if !sessions.is_empty() {
        open_session_windows(cx, services, sessions, handle.into());
    }
    Some(handle)
}

/// A window for a tab dragged out of another one, at `bounds` (screen coordinates), whatever
/// single window mode says: the drag asked for a window.
pub(crate) fn open_moved_tab_window(
    cx: &mut App,
    services: &Services,
    bounds: Bounds<gpui::Pixels>,
    let path = moved.first_folder()?.to_path_buf();
    let display = cx
        .displays()
        .iter()
        .find(|d| d.bounds().contains(&bounds.center()))
        .map(|d| d.id());
    let start = WindowStart {
        path,
        default: false,
        session: None,
        bounds: Some(WindowBounds::Windowed(bounds)),
        display,
        focus: true,
    };
    let handle = open_workspace_window(cx, services, start, None, Instant::now(), None)?;
    handle
        .update(cx, |ws, window, cx| ws.adopt_first(moved, window, cx))
        .ok()?;
    Some(handle)
}

/// What a new window starts on, and where.
struct WindowStart {
    path: PathBuf,
    /// No path was given: once settings load, go where "New windows open to" says.
    default: bool,
    /// Tabs to restore (the first opens on `path`).
    /// `None`: centered, at the default size.
    /// The screen `bounds` are on; `None`: the main one.
    /// Takes the foreground; the other windows of a restored session open behind.
    focus: bool,
}

fn open_workspace_window(
    cx: &mut App,
    services: &Services,
    start: WindowStart,
    started_at: Instant,
    let services = services.clone();
    let window_bounds = start.bounds.unwrap_or_else(|| {
        WindowBounds::Windowed(Bounds::centered(None, default_window_size(cx), cx))
    });
    let opened = cx.open_window(
            window_bounds: Some(window_bounds),
            // The title bar is drawn by us (the frame's top bar); the system keeps only the border
            // and snap layouts.
                title: Some("Arke".into()),
                appears_transparent: true,
            }),
            focus: start.focus,
            display_id: start.display,
            window_min_size: Some(size(theme::scaled_px(720.), theme::scaled_px(480.))),
            // top.
            window_background: if theme::mica() {
            } else {
            },
        },
        move |window, cx| {
            cx.new(|cx| {
                let mut workspace = Workspace::new(
                    services,
                    start.path,
                    start.default,
                    start.session,
                    bench,
                    started_at,
                    window,
                    cx,
                );
                // Before the first frame, which `open_window` draws before it returns.
                if let Some(received) = received {
                    workspace.time_from_request(received);
                }
                workspace
            })
        },
    );
    match opened {
            log::error!("cannot open window: {err:#}");
            None
        }
    }
}

#[cfg(test)]
mod arg_tests {
    use super::*;

    fn parse(args: &[&str]) -> Args {
    }

    #[test]
    fn the_shells_drive_root_argument_gets_its_backslash_back() {
        // `"arke.exe" "C:\"` arrives as `C:"`.
        assert_eq!(parse(&["C:\""]).path, Some(PathBuf::from(r"C:\")));
        assert_eq!(
        );
        assert_eq!(
        );
        assert_eq!(parse(&[r"C:\"]).path, Some(PathBuf::from(r"C:\")));
    }

    #[test]
    fn home_and_restore_flags_are_not_paths() {
        let args = parse(&["--home"]);
        assert!(args.page.is_none() && args.path.is_none() && !args.restore_explorer);
        let args = parse(&["--restore-explorer"]);
        assert!(args.restore_explorer && args.path.is_none() && args.page.is_none());
    }

    #[test]
    fn new_window_is_a_flag_not_a_path() {
        let args = parse(&["--new-window", r"C:\work"]);
        assert!(args.new_window);
        assert_eq!(args.path, Some(PathBuf::from(r"C:\work")));
        assert!(!parse(&[r"C:\work"]).new_window);
    }

    #[test]
    fn an_empty_argument_names_no_folder() {
        assert_eq!(parse(&[""]).path, None);
        assert_eq!(
            parse(&["", r"C:\work"]).path,
            Some(PathBuf::from(r"C:\work"))
        );
    }

    /// says, Home when that is the last tabs (they are open already).
    #[test]
    fn a_tab_with_no_folder_follows_the_startup_setting() {
        use settings::StartLocation;
        let home = || PathBuf::from(r"C:\Users\me");
        let last = Some(PathBuf::from(r"C:\last"));
        assert_eq!(
            fresh_tab(StartLocation::HomePage, last.clone(), home()),
            Target::Home
        );
        assert_eq!(
            fresh_tab(StartLocation::LastSession, last.clone(), home()),
            Target::Home
        );
        assert_eq!(
            fresh_tab(StartLocation::Home, last.clone(), home()),
            Target::Folder(home())
        );
        assert_eq!(
            fresh_tab(StartLocation::LastFolder, last, home()),
            Target::Folder(PathBuf::from(r"C:\last"))
        );
        assert_eq!(
            fresh_tab(StartLocation::LastFolder, None, home()),
            Target::Folder(home())
        );
    }

    #[test]
    fn bench_index_runs_without_a_window_bench() {
        let args = parse(&["--bench", "index", "--bench-out", r"C:\out.txt"]);
        assert_eq!(args.bench, Some(BenchArg::Index));
        assert_eq!(args.window_bench(), None);
        assert_eq!(args.bench_out, Some(PathBuf::from(r"C:\out.txt")));
        assert_eq!(args.path, None);
    }

    #[test]
    fn window_benches_still_parse() {
        let args = parse(&[r"C:\big", "--bench", "scroll"]);
        assert_eq!(args.path, Some(PathBuf::from(r"C:\big")));
    }

    #[test]
    fn folders_from_other_apps_go_to_the_window_used_last() {
        assert_eq!(pick_tab_host(&[1, 2, 3], Some(1)), Some(1));
        // The window used last was closed, or none was active yet: the newest.
        assert_eq!(pick_tab_host(&[1, 2, 3], Some(9)), Some(3));
        assert_eq!(pick_tab_host(&[1, 2], None), Some(2));
        // No window (only the notification area icon): open one.
        assert_eq!(pick_tab_host::<u8>(&[], Some(1)), None);
    }

    fn screen(x: f32, y: f32, width: f32, height: f32) -> Bounds<gpui::Pixels> {
        Bounds::new(gpui::point(px(x), px(y)), size(px(width), px(height)))
    }

    fn place(x: f32, y: f32, maximized: bool) -> settings::WindowPlace {
        settings::WindowPlace {
            x,
            y,
            width: 1000.,
            height: 700.,
            maximized,
        }
    }

    /// A window saved on the second screen opens there: gpui is told which screen, or Windows
    /// puts it at the main screen's default spot.
    #[test]
    fn a_window_comes_back_on_the_screen_it_was_on() {
        let main = screen(0., 0., 2560., 1440.);
        let second = screen(2560., 0., 1920., 1080.);
        let screens = [(one, main), (two, second)];
        let on_second = place_on_screens(place(2700., 100., false), &screens, Some(main)).unwrap();
        assert_eq!(on_second.display, Some(two));
        assert_eq!(
            on_second.bounds,
            WindowBounds::Windowed(screen(2700., 100., 1000., 700.))
        );
        let maximized = place_on_screens(place(2700., 100., true), &screens, Some(main)).unwrap();
        assert_eq!(maximized.display, Some(two));
        // Across the edge: the screen that shows more of it.
        let across = place_on_screens(place(2200., 100., false), &screens, Some(main)).unwrap();
        assert_eq!(across.display, Some(two));
        let across = place_on_screens(place(1800., 100., false), &screens, Some(main)).unwrap();
        assert_eq!(across.display, Some(one));
    }

    /// The screen a window was on is gone: a window gets the default spot, a maximized one is
    /// maximized on the main screen.
    #[test]
    fn a_window_whose_screen_is_gone_falls_back_to_the_main_one() {
        let main = screen(0., 0., 2560., 1440.);
        assert_eq!(
            place_on_screens(place(3000., 100., false), &screens, Some(main)),
            None
        );
        let maximized = place_on_screens(place(3000., 100., true), &screens, Some(main)).unwrap();
        assert_eq!(maximized.display, None);
        assert_eq!(
            maximized.bounds,
        );
        assert_eq!(
            place_on_screens(place(2500., 100., false), &screens, Some(main)),
            None
        );
        // Small saved sizes grow to the window's minimum.
        let small = settings::WindowPlace {
            width: 100.,
            height: 100.,
            ..place(10., 10., false)
        };
        assert_eq!(
            place_on_screens(small, &screens, Some(main)).map(|r| r.bounds),
            Some(WindowBounds::Windowed(screen(10., 10., 720., 480.)))
        );
    }
}
