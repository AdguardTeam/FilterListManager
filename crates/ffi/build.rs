fn main() {
    #[cfg(all(windows, feature = "win-res"))]
    {
        let _ = windres::Build::new().compile("resources/AGWinFLM.rc");
    }
}
