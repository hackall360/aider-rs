pub async fn start() {
    // placeholder sidecar
}

/// Expose onboarding logic to Flutter/Dart front-ends.
pub async fn select_model() -> Option<String> {
    aider_core::try_to_select_default_model().await
}
