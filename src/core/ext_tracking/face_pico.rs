use super::unified::{UnifiedExpressions, UnifiedShapeAccessors, UnifiedShapes, NUM_SHAPES};

#[allow(non_snake_case, unused)]
#[repr(usize)]
enum FacePico {
    EyeLookDownL,
    NoseSneerL,
    EyeLookInL,
    BrowInnerUp,
    BrowDownR,
    MouthClose,
    MouthLowerDownR,
    JawOpen,
    MouthUpperUpR,
    MouthShrugUpper,
    MouthFunnel,
    EyeLookInR,
    EyeLookDownR,
    NoseSneerR,
    MouthRollUpper,
    JawRight,
    BrowDownL,
    MouthShrugLower,
    MouthRollLower,
    MouthSmileL,
    MouthPressL,
    MouthSmileR,
    MouthPressR,
    MouthDimpleR,
    MouthLeft,
    JawForward,
    EyeSquintL,
    MouthFrownL,
    EyeBlinkL,
    CheekSquintL,
    BrowOuterUpL,
    EyeLookUpL,
    JawLeft,
    MouthStretchL,
    MouthPucker,
    EyeLookUpR,
    BrowOuterUpR,
    CheekSquintR,
    EyeBlinkR,
    MouthUpperUpL,
    MouthFrownR,
    EyeSquintR,
    MouthStretchR,
    CheekPuff,
    EyeLookOutL,
    EyeLookOutR,
    EyeWideR,
    EyeWideL,
    MouthRight,
    MouthDimpleL,
    MouthLowerDownL,
    TongueOut,
    VisemePP,
    VisemeCH,
    Visemeo,
    VisemeO,
    VisemeI,
    Visemeu,
    VisemeRR,
    VisemeXX,
    Visemeaa,
    Visemei,
    VisemeFF,
    VisemeU,
    VisemeTH,
    Visemekk,
    VisemeSS,
    Visemee,
    VisemeDD,
    VisemeE,
    Visemenn,
    Visemesil,
    FaceMax,
}

pub(crate) fn face_pico_to_unified(face_pico: &[f32]) -> Option<UnifiedShapes> {
    let mut shapes: UnifiedShapes = [0.0; NUM_SHAPES];
    if face_pico.len() < FacePico::FaceMax as usize {
        log::warn!(
            "Face tracking data is too short: {} < {}",
            face_pico.len(),
            FacePico::FaceMax as usize
        );
        return None;
    }

    let getf = |index| face_pico[index as usize];

    shapes.setu(
        UnifiedExpressions::EyeRightX,
        getf(FacePico::EyeLookOutR) - getf(FacePico::EyeLookInR),
    );
    shapes.setu(
        UnifiedExpressions::EyeLeftX,
        getf(FacePico::EyeLookOutL) - getf(FacePico::EyeLookInL),
    );
    shapes.setu(
        UnifiedExpressions::EyeY,
        getf(FacePico::EyeLookUpR) - getf(FacePico::EyeLookDownR),
    );

    let openness_l = 1.0
        - (getf(FacePico::EyeBlinkL) + getf(FacePico::EyeBlinkL) * getf(FacePico::EyeSquintL))
            .clamp(0.0, 1.0);
    let openness_r = 1.0
        - (getf(FacePico::EyeBlinkR) + getf(FacePico::EyeBlinkR) * getf(FacePico::EyeSquintR))
            .clamp(0.0, 1.0);
    shapes.setu(UnifiedExpressions::EyeClosedLeft, openness_l);
    shapes.setu(UnifiedExpressions::EyeClosedRight, openness_r);

    shapes.setu(
        UnifiedExpressions::EyeSquintRight,
        getf(FacePico::EyeSquintR),
    );
    shapes.setu(
        UnifiedExpressions::EyeSquintLeft,
        getf(FacePico::EyeSquintL),
    );
    shapes.setu(UnifiedExpressions::EyeWideRight, getf(FacePico::EyeWideR));
    shapes.setu(UnifiedExpressions::EyeWideLeft, getf(FacePico::EyeWideL));

    shapes.setu(
        UnifiedExpressions::BrowPinchRight,
        getf(FacePico::BrowDownR),
    );
    shapes.setu(UnifiedExpressions::BrowPinchLeft, getf(FacePico::BrowDownL));
    shapes.setu(
        UnifiedExpressions::BrowLowererRight,
        getf(FacePico::BrowDownR),
    );
    shapes.setu(
        UnifiedExpressions::BrowLowererLeft,
        getf(FacePico::BrowDownL),
    );
    shapes.setu(
        UnifiedExpressions::BrowInnerUpRight,
        getf(FacePico::BrowInnerUp),
    );
    shapes.setu(
        UnifiedExpressions::BrowInnerUpLeft,
        getf(FacePico::BrowInnerUp),
    );
    shapes.setu(
        UnifiedExpressions::BrowOuterUpRight,
        getf(FacePico::BrowOuterUpR),
    );
    shapes.setu(
        UnifiedExpressions::BrowOuterUpLeft,
        getf(FacePico::BrowOuterUpL),
    );

    shapes.setu(
        UnifiedExpressions::CheekSquintRight,
        getf(FacePico::CheekSquintR),
    );
    shapes.setu(
        UnifiedExpressions::CheekSquintLeft,
        getf(FacePico::CheekSquintL),
    );
    shapes.setu(
        UnifiedExpressions::CheekPuffRight,
        getf(FacePico::CheekPuff),
    );
    shapes.setu(UnifiedExpressions::CheekPuffLeft, getf(FacePico::CheekPuff));
    // shapes.setu(UnifiedExpressions::CheekSuckRight, getf(FaceFb::CheekSuckR));
    // shapes.setu(UnifiedExpressions::CheekSuckLeft, getf(FaceFb::CheekSuckL));

    shapes.setu(UnifiedExpressions::JawOpen, getf(FacePico::JawOpen));
    shapes.setu(UnifiedExpressions::JawRight, getf(FacePico::JawRight));
    shapes.setu(UnifiedExpressions::JawLeft, getf(FacePico::JawLeft));
    shapes.setu(UnifiedExpressions::JawForward, getf(FacePico::JawForward));
    shapes.setu(UnifiedExpressions::MouthClosed, getf(FacePico::MouthClose));

    shapes.setu(
        UnifiedExpressions::LipSuckUpperRight,
        getf(FacePico::MouthRollUpper),
    );
    shapes.setu(
        UnifiedExpressions::LipSuckUpperLeft,
        getf(FacePico::MouthRollUpper),
    );

    shapes.setu(
        UnifiedExpressions::LipSuckLowerRight,
        getf(FacePico::MouthRollLower),
    );
    shapes.setu(
        UnifiedExpressions::LipSuckLowerLeft,
        getf(FacePico::MouthRollLower),
    );
    shapes.setu(
        UnifiedExpressions::LipFunnelUpperRight,
        getf(FacePico::MouthFunnel),
    );
    shapes.setu(
        UnifiedExpressions::LipFunnelUpperLeft,
        getf(FacePico::MouthFunnel),
    );
    shapes.setu(
        UnifiedExpressions::LipFunnelLowerRight,
        getf(FacePico::MouthFunnel),
    );
    shapes.setu(
        UnifiedExpressions::LipFunnelLowerLeft,
        getf(FacePico::MouthFunnel),
    );
    shapes.setu(
        UnifiedExpressions::LipPuckerUpperRight,
        getf(FacePico::MouthPucker),
    );
    shapes.setu(
        UnifiedExpressions::LipPuckerUpperLeft,
        getf(FacePico::MouthPucker),
    );
    shapes.setu(
        UnifiedExpressions::LipPuckerLowerRight,
        getf(FacePico::MouthPucker),
    );
    shapes.setu(
        UnifiedExpressions::LipPuckerLowerLeft,
        getf(FacePico::MouthPucker),
    );

    shapes.setu(
        UnifiedExpressions::NoseSneerRight,
        getf(FacePico::NoseSneerR),
    );
    shapes.setu(
        UnifiedExpressions::NoseSneerLeft,
        getf(FacePico::NoseSneerL),
    );

    shapes.setu(
        UnifiedExpressions::MouthLowerDownRight,
        getf(FacePico::MouthLowerDownR),
    );
    shapes.setu(
        UnifiedExpressions::MouthLowerDownLeft,
        getf(FacePico::MouthLowerDownL),
    );

    let mouth_upper_up_deepen_right =
        0_f32.max(getf(FacePico::MouthUpperUpR) - getf(FacePico::NoseSneerR));
    let mouth_upper_up_deepen_left =
        0_f32.max(getf(FacePico::MouthUpperUpL) - getf(FacePico::NoseSneerL));

    shapes.setu(
        UnifiedExpressions::MouthUpperUpRight,
        mouth_upper_up_deepen_right,
    );
    shapes.setu(
        UnifiedExpressions::MouthUpperUpLeft,
        mouth_upper_up_deepen_left,
    );
    shapes.setu(
        UnifiedExpressions::MouthUpperDeepenRight,
        mouth_upper_up_deepen_right,
    );
    shapes.setu(
        UnifiedExpressions::MouthUpperDeepenLeft,
        mouth_upper_up_deepen_left,
    );

    shapes.setu(
        UnifiedExpressions::MouthUpperRight,
        getf(FacePico::MouthRight),
    );
    shapes.setu(
        UnifiedExpressions::MouthUpperLeft,
        getf(FacePico::MouthLeft),
    );
    shapes.setu(
        UnifiedExpressions::MouthLowerRight,
        getf(FacePico::MouthRight),
    );
    shapes.setu(
        UnifiedExpressions::MouthLowerLeft,
        getf(FacePico::MouthLeft),
    );

    shapes.setu(
        UnifiedExpressions::MouthCornerPullRight,
        getf(FacePico::MouthSmileR),
    );
    shapes.setu(
        UnifiedExpressions::MouthCornerPullLeft,
        getf(FacePico::MouthSmileL),
    );
    shapes.setu(
        UnifiedExpressions::MouthCornerSlantRight,
        getf(FacePico::MouthSmileR),
    );
    shapes.setu(
        UnifiedExpressions::MouthCornerSlantLeft,
        getf(FacePico::MouthSmileL),
    );

    shapes.setu(
        UnifiedExpressions::MouthFrownRight,
        getf(FacePico::MouthFrownR),
    );
    shapes.setu(
        UnifiedExpressions::MouthFrownLeft,
        getf(FacePico::MouthFrownL),
    );
    shapes.setu(
        UnifiedExpressions::MouthStretchRight,
        getf(FacePico::MouthStretchR),
    );
    shapes.setu(
        UnifiedExpressions::MouthStretchLeft,
        getf(FacePico::MouthStretchR),
    );

    shapes.setu(
        UnifiedExpressions::MouthDimpleLeft,
        getf(FacePico::MouthDimpleL),
    );
    shapes.setu(
        UnifiedExpressions::MouthDimpleRight,
        getf(FacePico::MouthDimpleR),
    );

    shapes.setu(
        UnifiedExpressions::MouthRaiserUpper,
        getf(FacePico::MouthShrugUpper),
    );
    shapes.setu(
        UnifiedExpressions::MouthRaiserLower,
        getf(FacePico::MouthShrugLower),
    );
    shapes.setu(
        UnifiedExpressions::MouthPressRight,
        getf(FacePico::MouthPressR),
    );
    shapes.setu(
        UnifiedExpressions::MouthPressLeft,
        getf(FacePico::MouthPressL),
    );
    // shapes.setu(
    //     UnifiedExpressions::MouthTightenerRight,
    //     getf(FaceFb::LipTightenerR),
    // );
    // shapes.setu(
    //     UnifiedExpressions::MouthTightenerLeft,
    //     getf(FaceFb::LipTightenerL),
    // );

    shapes.setu(UnifiedExpressions::TongueOut, getf(FacePico::TongueOut));

    Some(shapes)
}
