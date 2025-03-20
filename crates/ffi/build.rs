fn main() {
    #[cfg(windows)]
    {
        let _ = windres::Build::new().compile("resources/AGWinFLM.rc");
    }
}
