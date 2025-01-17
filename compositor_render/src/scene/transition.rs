use std::time::Duration;

use super::{types::interpolation::InterpolationState, InterpolationKind};

/// Similar concept to InterpolationState, but it represents a time instead.
/// Values between 0 and 1 represent transition and larger than 1 post transition.
///
/// If interpolation_kind is linear then InterpolationState and TransitionProgress
/// have the same numerical value.
#[derive(Debug, Clone, Copy)]
struct TransitionProgress(f64);

#[derive(Debug, Clone)]
pub(super) struct TransitionState {
    /// Additional offset for transition. It is non zero if you want to start
    /// a transition in the middle of the interpolation curve.
    initial_offset: (TransitionProgress, InterpolationState),

    // PTS of a first frame of transition.
    start_pts: Duration,

    /// Duration of the transition.
    duration: Duration,

    interpolation_kind: InterpolationKind,
}

pub(super) struct TransitionOptions {
    pub duration: Duration,
    pub interpolation_kind: InterpolationKind,
}

impl TransitionState {
    pub fn new(
        current_transition: Option<TransitionOptions>,
        previous_transition: Option<TransitionState>,
        last_pts: Duration,
    ) -> Option<Self> {
        let previous_transition = previous_transition.and_then(|transition| {
            if transition.start_pts + transition.duration < last_pts {
                return None;
            }
            Some(transition)
        });
        match (current_transition, previous_transition) {
            (None, None) => None,
            (None, Some(previous_transition)) => {
                let remaining_duration = (previous_transition.start_pts
                    + previous_transition.duration)
                    .saturating_sub(last_pts);
                let progress_offset = TransitionProgress(
                    1.0 - (remaining_duration.as_secs_f64()
                        / previous_transition.duration.as_secs_f64()),
                );
                let state_offset = previous_transition
                    .interpolation_kind
                    .state(progress_offset.0);
                Some(Self {
                    initial_offset: (progress_offset, state_offset),
                    start_pts: last_pts,
                    duration: remaining_duration,
                    interpolation_kind: previous_transition.interpolation_kind,
                })
            }
            (Some(current_transition), _) => Some(Self {
                initial_offset: (TransitionProgress(0.0), InterpolationState(0.0)),
                start_pts: last_pts,
                duration: current_transition.duration,
                interpolation_kind: current_transition.interpolation_kind,
            }),
        }
    }

    pub fn state(&self, pts: Duration) -> InterpolationState {
        // Value in range [0, 1], where 1 means end of transition.
        let progress =
            (pts.as_secs_f64() - self.start_pts.as_secs_f64()) / self.duration.as_secs_f64();
        // Value in range [initial_offset.0 , 1]. Previous progress ([0, 1]) is rescaled to fit
        // smaller ranger and offset is added.
        let progress = self.initial_offset.0 .0 + progress * (1.0 - self.initial_offset.0 .0);
        // Clamp just to handle a case where this function is called after transition is finished.
        let progress = f64::clamp(progress, 0.0, 1.0);
        // Value in range [initial_offset.1, 1] or [state(initial_offset.0), 1].
        let state = self.interpolation_kind.state(progress);
        // Value in range [0, 1].
        InterpolationState((state.0 - self.initial_offset.1 .0) / (1.0 - self.initial_offset.1 .0))
    }
}

impl InterpolationKind {
    fn state(&self, t: f64) -> InterpolationState {
        match self {
            InterpolationKind::Linear => InterpolationState(t),
        }
    }
}
