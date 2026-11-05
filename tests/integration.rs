use sigmaos_arch_security_tracker::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_security_tracker::ArchSecurityTracker::new("sigmaos-arch-security-tracker").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-security-tracker"));
}
