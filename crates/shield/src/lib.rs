pub struct OperatorShield {
    damping_factor: f32,
    last_safe_state: f32,
}

impl OperatorShield {
    pub fn new(damping: f32) -> Self {
        Self { damping_factor: damping, last_safe_state: 0.0 }
    }

    /// Sanitizes raw operator input to prevent "glitching" or "panic" states
    pub fn sanitize(&mut self, input: f32) -> f32 {
        if input.is_nan() || input.is_infinite() {
            return self.last_safe_state;
        }
        
        // Apply Sigmoid Damping to prevent rapid jitter
        let smoothed = self.last_safe_state + (input - self.last_safe_state) * self.damping_factor;
        self.last_safe_state = smoothed;
        smoothed
    }
}
