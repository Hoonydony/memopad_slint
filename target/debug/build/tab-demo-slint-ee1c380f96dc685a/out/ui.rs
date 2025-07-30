# [allow (non_snake_case , non_camel_case_types)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] mod slint_generatedTabDemo {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerCupertinoPalette_127 {
         r#background : sp :: Property < slint :: Brush > , r#border : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , r#foreground_secondary : sp :: Property < slint :: Brush > , r#selection_background : sp :: Property < slint :: Brush > , r#selection_foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerCupertinoPalette_127 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#background }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4280690214f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4293980400f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) . transparentize (0.9f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280032286f64 as u32)) . transparentize (0.9f64 as f32) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_CupertinoPalette_127_color_scheme = InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () ;
                         ;
                         if (! ((r#tmp_CupertinoPalette_127_color_scheme . clone ()) == (sp :: r#ColorScheme :: r#Unknown))) {
                             (((r#tmp_CupertinoPalette_127_color_scheme . clone ()) == (sp :: r#ColorScheme :: r#Dark))) as _ }
                         else {
                             {
                                 ((sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . color_scheme ()) == (sp :: r#ColorScheme :: r#Dark)) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) . transparentize (0.4f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280032286f64 as u32)) . transparentize (0.4f64 as f32) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_background }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) . transparentize (0.5f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) . transparentize (0.5f64 as f32) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    )) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_1 {
         r#root_1 : sp :: r#Empty , r#_opacity_2 : sp :: r#Opacity , r#rectangle_3 : sp :: r#BasicBorderRectangle , r#i_touch_area_26 : sp :: r#TouchArea , r#i_focus_scope_27 : sp :: r#FocusScope , r#root_1_background : sp :: Property < slint :: Brush > , r#root_1_checked : sp :: Property < bool > , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_layout_19_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_i_layout_19_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_i_layout_19_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_icon : sp :: Property < sp :: Image > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_state : sp :: Property < i32 > , r#root_1_text : sp :: Property < sp :: SharedString > , r#root_1_width : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_accessible_action_default : sp :: Callback < () , () > , r#root_1_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent__shadow_4 > , repeater1 : sp :: Repeater < InnerComponent__shadow_13 > , repeater2 : sp :: Repeater < InnerComponent__opacity_20 > , repeater3 : sp :: Repeater < InnerComponent__opacity_23 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (false as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (true as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ((((((sp :: Image :: default () . size ()) . r#width as f64) > ((0f64 as i32) as f64))) && ((((sp :: Image :: default () . size ()) . r#height as f64) > ((0f64 as i32) as f64)))) as bool)) as _ }
                 }
            ) ;
             _self . repeater3 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                    ) . apply_pin (_self) . get ()) != (sp :: SharedString :: from (""))) as bool)) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_state = ({
                             * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded (4284703590f64 as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (2f64 as f64)) {
                                 (if false {
                                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded (4281494735f64 as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded (4287532691f64 as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded (4287532691f64 as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if ((false) && (({
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) {
                                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32))) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) }
                                    ) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded (4284703590f64 as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                                    ) }
                                 }
                             }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 8f64 as _ ;
                                 the_struct . r#end = 8f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater2 . len () + _self . repeater3 . len ()) ;
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater3 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 4f64 as _ ;
                             the_struct . r#end = 4f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    ) + (({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    ) + (({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (! ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) {
                         (1f64) as _ }
                     else {
                         if ((({
                             * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                         + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                             * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                             (2f64) as _ }
                         else {
                             if ({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                            ) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 0f64 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (0.5f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4281494735f64 as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     ({
                                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                                    ) . apply_pin (_self) . set ((! ({
                                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                                    ) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             ({
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (! (((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n")))))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     ({
                                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_4 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 3u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater3 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (20f64 as sp :: Coord) . max (((({
                             * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_4 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 3u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater3 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_4 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent__shadow_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_20 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 3u32 => {
                     InnerButton_root_1 :: FIELD_OFFSETS . repeater3 . apply_pin (_self) . ensure_updated (|| InnerComponent__opacity_23 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater3 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 6u32 => (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 7u32 => (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 8u32 => (((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_4 {
         r#_shadow_4 : sp :: r#BoxShadow , r#rectangle_5 : sp :: r#BasicBorderRectangle , r#_shadow_6 : sp :: r#BoxShadow , r#rectangle_7 : sp :: r#BasicBorderRectangle , r#_shadow_8 : sp :: r#BoxShadow , r#rectangle_9 : sp :: r#BasicBorderRectangle , r#_opacity_10 : sp :: r#Opacity , r#rectangle_11 : sp :: r#BasicBorderRectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_4 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_4 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_opacity_10 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.17f64) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294967295f64 as u32) , position : 1f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4278190080f64 as u32) , position : 0f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , sp :: Orientation :: Vertical => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 6u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 7u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent__shadow_4 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             8usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 8u32 , parent_index : 6u32 , item_array_index : 7u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_4 , sp :: ItemVTable , sp :: AllowPin > ;
             8usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_4 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_6 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_7 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_shadow_8 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_9 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#_opacity_10 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_4 :: FIELD_OFFSETS . r#rectangle_11 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_4) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_4 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_4 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_4 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_4 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 2u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__shadow_4 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_4 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__shadow_13 {
         r#_shadow_13 : sp :: r#BoxShadow , r#rectangle_14 : sp :: r#BasicBorderRectangle , r#_opacity_15 : sp :: r#Opacity , r#_shadow_16 : sp :: r#BoxShadow , r#rectangle_17 : sp :: r#BasicBorderRectangle , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__shadow_13 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__shadow_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_opacity_15 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () {
                         (1f64) as _ }
                     else {
                         0.5f64 }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (637534208f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_background) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4280032286f64 as u32)) . transparentize (0.9f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) . transparentize (0.9f64 as f32) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , sp :: Orientation :: Vertical => {
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent__shadow_13 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             5usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__shadow_13 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_13 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_14 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_opacity_15 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#_shadow_16 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__shadow_13 :: FIELD_OFFSETS . r#rectangle_17 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__shadow_13) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__shadow_13 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__shadow_13 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__shadow_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__shadow_13 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__shadow_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__shadow_13 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_20 {
         r#_opacity_20 : sp :: r#Opacity , r#image_21 : sp :: r#ImageItem , r#_opacity_20_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_20_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_20 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_20 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | sp :: Property :: link_two_way (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () {
                         (1f64) as _ }
                     else {
                         0.5f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 13f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 13f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 13f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent__opacity_20 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_20 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#_opacity_20 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_20 :: FIELD_OFFSETS . r#image_21 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_20) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_20 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_20 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_20 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_20 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__opacity_20 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_20 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent__opacity_23 {
         r#_opacity_23 : sp :: r#Opacity , r#text_24 : sp :: r#SimpleText , r#_opacity_23_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#_opacity_23_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent__opacity_23 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent__opacity_23 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: Item :: layout_info (({
                         * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if _self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () {
                         (1f64) as _ }
                     else {
                         0.5f64 }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_1_state = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_state) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () ;
                         ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (1f64 as f64)) {
                             (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_1_state . clone () as f64) , & (3f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded (4281494735f64 as u32) }
                                )) as _ }
                             else {
                                 if ((false) && (_self . parent . upgrade () . as_ref () . map (| x | ({
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ())) {
                                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293388263f64 as u32))) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded (4291940822f64 as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded (4281084974f64 as u32) }
                                    ) }
                                 }
                             }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . window_item () . unwrap () . as_pin_ref () . default_font_size () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 ((400f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 4f64 as sp :: Coord ,) , 1u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_19_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent__opacity_23 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_1 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent__opacity_23 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#_opacity_23 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent__opacity_23 :: FIELD_OFFSETS . r#text_24 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent__opacity_23) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent__opacity_23 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent__opacity_23 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent__opacity_23 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent__opacity_23 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 5u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent__opacity_23 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent__opacity_23 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEditBase_root_28 {
         r#root_28 : sp :: r#Empty , r#root_clip_33 : sp :: r#Clip , r#placeholder_34 : sp :: r#ComplexText , r#contextmenuinternal_35 : sp :: r#ContextMenu , r#text_input_36 : sp :: r#TextInput , r#root_28_contextmenuinternal_35_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_28_has_focus : sp :: Property < bool > , r#root_28_height : sp :: Property < sp :: LogicalLength > , r#root_28_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_28_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_28_margin : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_horizontal_stretch : sp :: Property < f32 > , r#root_28_placeholder_34_max_height : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_max_width : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_min_height : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_min_width : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_28_placeholder_34_vertical_stretch : sp :: Property < f32 > , r#root_28_placeholder_color : sp :: Property < slint :: Brush > , r#root_28_placeholder_text : sp :: Property < sp :: SharedString > , r#root_28_text_color : sp :: Property < slint :: Brush > , r#root_28_text_input_36_computed_x : sp :: Property < sp :: LogicalLength > , r#root_28_text_input_36_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_28_text_input_36_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_28_width : sp :: Property < sp :: LogicalLength > , r#root_28_x : sp :: Property < sp :: LogicalLength > , r#root_28_accepted : sp :: Callback < (sp :: SharedString ,) , () > , r#root_28_edited : sp :: Callback < (sp :: SharedString ,) , () > , r#root_28_key_pressed : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , r#root_28_key_released : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEditBase_root_28 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEditBase_root_28 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_contextmenuinternal_35_entries }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! [{
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("1") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Cut")) as _ , (sp :: SharedString :: from ("LineEditBase")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("2") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Copy")) as _ , (sp :: SharedString :: from ("LineEditBase")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("3") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Paste")) as _ , (sp :: SharedString :: from ("LineEditBase")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("4") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Select All")) as _ , (sp :: SharedString :: from ("LineEditBase")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _]))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_2 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_2 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_2 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = 1f64 as _ ;
                             the_struct . r#min_percent = (r#layout_info_2 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_2 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_2 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_3 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_3 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_3 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_height }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct . r#min_percent = (r#layout_info_3 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_3 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_3 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    ) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_34_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         (({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_text }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         (if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("1"))) {
                             ({
                                 ({
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                ) . apply_pin (_self) . r#cut ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) }
                            ) ;
                             }
                         else {
                             if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("2"))) {
                                 ({
                                     ({
                                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                    ) . apply_pin (_self) . r#copy ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) }
                                ) ;
                                 }
                             else {
                                 if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("3"))) {
                                     ({
                                         ({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                        ) . apply_pin (_self) . r#paste ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) }
                                    ) ;
                                     }
                                 else {
                                     if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("4"))) {
                                         ({
                                             ({
                                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                            ) . apply_pin (_self) . r#select_all ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1)) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_115 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let entries = ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_contextmenuinternal_35_entries }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                ) . apply_pin (_self) . set (entries) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                                        ) . apply_pin (_self) . sub_menu . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_activated }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                                        ) . apply_pin (_self) . activated . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                                ) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = ({
                                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                                    ) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             if let Some (current_id) = ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                            ) . apply_pin (_self) . popup_id . take () {
                                 sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                 }
                             let id = sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , true ,) ;
                             ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                            ) . apply_pin (_self) . popup_id . set (Some (id)) ;
                             InnerPopupMenuImpl_root_115 :: user_init (popup_instance_vrc) ;
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#sub_menu) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! []))) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_accepted }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_color }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
                            ) . apply_pin (_self) . get () . get () as f64)) {
                                 ({
                                     ({
                                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_computed_x }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((((- (args . 0 . clone ()) . r#x) as f64) + (({
                                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
                                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (({
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as f64)) {
                                     ({
                                         ({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_computed_x }
                                        ) . apply_pin (_self) . set (sp :: LogicalLength :: new (((((((({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (({
                                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_key_pressed }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_released) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_key_released }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_background . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_foreground . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#page_height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 1f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = ({
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ((1f64 as f64) * (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (((1f64 as f64) * (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , ({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (0f64 as sp :: Coord) . min (((((((({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) . get () . get () as f64) - (({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) - (1f64 as f64)) as sp :: Coord) . max ((({
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_computed_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , false)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_selection (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . r#clear_selection ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_copy (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . r#copy ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_cut (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . r#cut ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , true)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_paste (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . r#paste ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_select_all (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . r#select_all ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1))) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_set_selection_offsets (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 , arg_1 : i32 ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             (({
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            ) . apply_pin (_self) . set_selection_offsets ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 4u32 - 1) , args . 0 . clone () as i32 , args . 1 . clone () as i32)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerLineEdit_root_37 {
         r#root_37 : sp :: r#Empty , r#_opacity_38 : sp :: r#Opacity , r#rectangle_39 : sp :: r#Rectangle , r#background_opacity_40 : sp :: r#Opacity , r#background_41 : sp :: r#BasicBorderRectangle , r#base_43 : InnerLineEditBase_root_28 , r#root_37_accessible_placeholder_text : sp :: Property < sp :: SharedString > , r#root_37_background_41_width : sp :: Property < sp :: LogicalLength > , r#root_37_height : sp :: Property < sp :: LogicalLength > , r#root_37_layout_42_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_37_layout_42_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_layout_42_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_37_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_37_state : sp :: Property < i32 > , r#root_37_width : sp :: Property < sp :: LogicalLength > , r#root_37_x : sp :: Property < sp :: LogicalLength > , r#root_37_y : sp :: Property < sp :: LogicalLength > , r#root_37_accessible_action_set_value : sp :: Callback < (sp :: SharedString ,) , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerLineEdit_root_37 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerLineEdit_root_37 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEditBase_root_28 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_43 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 6u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_action_set_value }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                             + {
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (args . 0 . clone () as _) ;
                             ({
                                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                             + {
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_edited }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_placeholder_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                     + {
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))) {
                         (({
                             InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                         + {
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_text }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_background_41_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                                 + {
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 1f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 7f64 as _ ;
                             the_struct . r#end = 7f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_background_41_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                             + {
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 1f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 7f64 as _ ;
                         the_struct . r#end = 7f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                             + {
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                                 + {
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_input_36_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_4 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_4 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_4 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_4 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_4 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_5 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_5 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_5 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_5 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_5 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (! ({
                         InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                     + {
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) {
                         (1f64) as _ }
                     else {
                         if ({
                             InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                         + {
                             * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_has_focus }
                        ) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#_opacity_38 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                     + {
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_has_focus }
                    ) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (0.5f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#rectangle_39 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4281494735f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_opacity_40 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                     + {
                         * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0.5f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_37_state = ({
                             * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_37_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded (4281084974f64 as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded (4293388263f64 as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_37_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded (4284703590f64 as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded (4284703590f64 as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#border . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . window_item () . unwrap () . as_pin_ref () . default_font_size () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 ((400f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_placeholder_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                     else {
                         InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_background . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    ) . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_margin }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEditBase_root_28 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_43 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                             * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 2u32 => (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 6u32 ..= 9u32 => return {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . item_geometry (index - 6u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#TextInput , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessible_role (0) , 6u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessible_role (index - 6u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (0u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (6u32 ..= 9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessible_string_property (index - 6u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     ({
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_action_set_value }
                    ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                 (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (6u32 ..= 9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . accessibility_action (index - 6u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#SetValue , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 6u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 6u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 6u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_43 }
                 . apply_pin (_self) . item_element_infos (index - 6u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerTabComponent_root_44 {
         r#root_44 : sp :: r#Empty , r#rectangle_45 : sp :: r#Rectangle , r#root_44_current_tab : sp :: Property < i32 > , r#root_44_editing : sp :: Property < bool > , r#root_44_height : sp :: Property < sp :: LogicalLength > , r#root_44_idx : sp :: Property < i32 > , r#root_44_tab_name : sp :: Property < sp :: SharedString > , r#root_44_tab_width : sp :: Property < sp :: LogicalLength > , r#root_44_width : sp :: Property < sp :: LogicalLength > , r#root_44_x : sp :: Property < sp :: LogicalLength > , r#root_44_y : sp :: Property < sp :: LogicalLength > , r#root_44_name_changed : sp :: Callback < (sp :: SharedString ,) , () > , r#root_44_select_tab : sp :: Callback < (i32 ,) , () > , repeater0 : sp :: Repeater < InnerComponent_btn_46 > , repeater1 : sp :: Repeater < InnerComponent_lineedit_48 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTabComponent_root_44 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTabComponent_root_44 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ((! ({
                         * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_editing }
                    ) . apply_pin (_self) . get ()) as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (({
                         * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_editing }
                    ) . apply_pin (_self) . get () as bool)) as _ }
                 }
            ) ;
             ({
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_editing }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#rectangle_45 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_idx }
                    ) . apply_pin (_self) . get () as f64) , & (({
                         * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_current_tab }
                    ) . apply_pin (_self) . get () as f64)) {
                         (sp :: Color :: from_argb_encoded (4291875024f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4293980400f64 as u32) }
                    )) as _ }
                ) ;
                 }
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_btn_46 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_lineedit_48 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_btn_46 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_lineedit_48 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_btn_46 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerTabComponent_root_44 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_lineedit_48 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (25f64 as sp :: Coord , ({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_width }
                ) . apply_pin (_self) . get () . get () as f64) - (({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_height }
                ) . apply_pin (_self) . get () . get () as f64) - (25f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_btn_46 {
         r#btn_46 : InnerButton_root_1 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_btn_46 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_btn_46 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#btn_46 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_select_tab) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_idx) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () as _ ,))) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_name) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#btn_46 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#btn_46 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#btn_46 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#btn_46 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (25f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , ({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 ..= 8u32 => return {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#btn_46 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_btn_46 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             9usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 4u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_btn_46 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_btn_46 :: FIELD_OFFSETS . r#btn_46 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_btn_46) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_btn_46 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_btn_46 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_btn_46 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_btn_46 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 2u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_btn_46 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_btn_46 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_lineedit_48 {
         r#lineedit_48 : InnerLineEdit_root_37 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_lineedit_48 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_lineedit_48 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEdit_root_37 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#lineedit_48 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_accepted }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_name) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (({
                                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                                 + {
                                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                                 + {
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _)) ;
                                 }
                             ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_editing) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (false as _)) ;
                                 }
                             ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_name_changed) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& (({
                                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                                 + {
                                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                                 + {
                                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,))) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (25f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_name) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_37 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#lineedit_48 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (25f64 as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , ({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 ..= 9u32 => return {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#TextInput , 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (0u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     ({
                         InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
                     + {
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_action_set_value }
                    ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#SetValue , 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_48 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_lineedit_48 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabComponent_root_44 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             10usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 4u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 7u32 , parent_index : 5u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 6u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 6u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 8u32 , item_array_index : 9u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_lineedit_48 , sp :: ItemVTable , sp :: AllowPin > ;
             10usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#_opacity_38 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_opacity_40 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#rectangle_39 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_lineedit_48 :: FIELD_OFFSETS . r#lineedit_48 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_lineedit_48) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_lineedit_48 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_lineedit_48 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_lineedit_48 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_lineedit_48 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 3u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_lineedit_48 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_lineedit_48 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerTextEdit_root_50 {
         r#root_50 : sp :: r#Empty , r#_opacity_55 : sp :: r#Opacity , r#rectangle_56 : sp :: r#Rectangle , r#background_57 : sp :: r#BasicBorderRectangle , r#contextmenuinternal_58 : sp :: r#ContextMenu , r#scroll_view_59 : sp :: r#Empty , r#flickable_60 : sp :: r#Flickable , r#flickable_viewport_61 : sp :: r#Empty , r#text_input_62 : sp :: r#TextInput , r#vertical_bar_visibility_63 : sp :: r#Clip , r#vertical_bar_64 : sp :: r#Rectangle , r#border_65 : sp :: r#Rectangle , r#thumb_opacity_66 : sp :: r#Opacity , r#thumb_67 : sp :: r#BasicBorderRectangle , r#touch_area_68 : sp :: r#TouchArea , r#horizontal_bar_visibility_69 : sp :: r#Clip , r#horizontal_bar_70 : sp :: r#Rectangle , r#border_71 : sp :: r#Rectangle , r#thumb_opacity_72 : sp :: r#Opacity , r#thumb_73 : sp :: r#BasicBorderRectangle , r#touch_area_74 : sp :: r#TouchArea , r#placeholder_75 : sp :: r#ComplexText , r#root_50_accessible_placeholder_text : sp :: Property < sp :: SharedString > , r#root_50_contextmenuinternal_58_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_50_flickable_60_height : sp :: Property < sp :: LogicalLength > , r#root_50_flickable_60_width : sp :: Property < sp :: LogicalLength > , r#root_50_height : sp :: Property < sp :: LogicalLength > , r#root_50_horizontal_bar_70_height : sp :: Property < sp :: LogicalLength > , r#root_50_horizontal_bar_70_maximum : sp :: Property < sp :: LogicalLength > , r#root_50_horizontal_bar_70_pad : sp :: Property < sp :: LogicalLength > , r#root_50_horizontal_bar_70_visible : sp :: Property < bool > , r#root_50_horizontal_bar_70_width : sp :: Property < sp :: LogicalLength > , r#root_50_placeholder_75_min_height : sp :: Property < sp :: LogicalLength > , r#root_50_placeholder_75_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_50_state : sp :: Property < i32 > , r#root_50_text_input_62_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_50_text_input_62_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_50_thumb_67_height : sp :: Property < sp :: LogicalLength > , r#root_50_thumb_67_width : sp :: Property < sp :: LogicalLength > , r#root_50_thumb_73_height : sp :: Property < sp :: LogicalLength > , r#root_50_thumb_73_width : sp :: Property < sp :: LogicalLength > , r#root_50_touch_area_68_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_50_touch_area_74_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_50_vertical_bar_64_height : sp :: Property < sp :: LogicalLength > , r#root_50_vertical_bar_64_maximum : sp :: Property < sp :: LogicalLength > , r#root_50_vertical_bar_64_pad : sp :: Property < sp :: LogicalLength > , r#root_50_vertical_bar_64_visible : sp :: Property < bool > , r#root_50_vertical_bar_64_width : sp :: Property < sp :: LogicalLength > , r#root_50_width : sp :: Property < sp :: LogicalLength > , r#root_50_x : sp :: Property < sp :: LogicalLength > , r#root_50_y : sp :: Property < sp :: LogicalLength > , r#root_50_edited : sp :: Callback < (sp :: SharedString ,) , () > , r#root_50_key_pressed : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , r#root_50_key_released : sp :: Callback < (sp :: KeyEvent ,) , sp :: r#EventResult > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTextEdit_root_50 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTextEdit_root_50 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_accessible_placeholder_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))) {
                         (sp :: SharedString :: from ("")) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_contextmenuinternal_58_entries }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! [{
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("1") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Cut")) as _ , (sp :: SharedString :: from ("TextEdit")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("2") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Copy")) as _ , (sp :: SharedString :: from ("TextEdit")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("3") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Paste")) as _ , (sp :: SharedString :: from ("TextEdit")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _ , {
                         let mut the_struct = sp :: MenuEntry :: default () ;
                         the_struct . r#has_sub_menu = false as _ ;
                         the_struct . r#id = sp :: SharedString :: from ("4") as _ ;
                         the_struct . r#title = slint :: private_unstable_api :: translate ((sp :: SharedString :: from ("Select All")) as _ , (sp :: SharedString :: from ("TextEdit")) as _ , (sp :: SharedString :: from ("tab-demo-slint")) as _ , (sp :: Slice :: from_slice (& [])) as _ , (1f64) as _ , (sp :: SharedString :: from ("")) as _) as _ ;
                         the_struct }
                     as _]))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1f64 as f64) * (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (16f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (20f64) as _ }
                     else {
                         12f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_visible }
                    ) . apply_pin (_self) . get () {
                         (((((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                        ) . apply_pin (_self) . get () . get () as f64))) as _ }
                     else {
                         ((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_placeholder_75_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_placeholder_75_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if (! ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                         + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_text_input_62_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_text_input_62_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_50_vertical_bar_64_maximum = ({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ;
                         ((if ((r#tmp_root_50_vertical_bar_64_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_50_vertical_bar_64_page_size = ({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                                ) . apply_pin (_self) . get () . get () ;
                                 ;
                                 (((32f64 as sp :: Coord) . max ((((((({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) * (((r#tmp_root_50_vertical_bar_64_page_size . clone () as f64) / (((r#tmp_root_50_vertical_bar_64_maximum . clone () as f64) + (r#tmp_root_50_vertical_bar_64_page_size . clone () as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                             }
                         as f64) / (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_pad }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64)) as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((2f64 as f64) * (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_pad }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64)) as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_50_horizontal_bar_70_maximum = ({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ;
                         ((if ((r#tmp_root_50_horizontal_bar_70_maximum . clone () as f64) <= (((0f64 as f64) / (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) as f64)) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_50_horizontal_bar_70_page_size = ({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                                ) . apply_pin (_self) . get () . get () ;
                                 ;
                                 (((32f64 as sp :: Coord) . max ((((((((({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) * (r#tmp_root_50_horizontal_bar_70_page_size . clone () as f64)) as f64) / (((r#tmp_root_50_horizontal_bar_70_maximum . clone () as f64) + (r#tmp_root_50_horizontal_bar_70_page_size . clone () as f64)) as f64)) as sp :: Coord)) as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                             }
                         as f64) / (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . scale_factor () as f64)) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_visible }
                    ) . apply_pin (_self) . get () {
                         (((((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                        ) . apply_pin (_self) . get () . get () as f64))) as _ }
                     else {
                         ((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                        ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_pad }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64) > (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                    ) . apply_pin (_self) . get () . get () as f64))) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (20f64) as _ }
                     else {
                         12f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#_opacity_55 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (0.5f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#rectangle_56 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4281494735f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_50_state = ({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_50_state . clone () as f64) , & (1f64 as f64)) {
                             (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                 (sp :: Color :: from_argb_encoded (4281084974f64 as u32)) as _ }
                             else {
                                 sp :: Color :: from_argb_encoded (4293388263f64 as u32) }
                            )) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_50_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded (4284703590f64 as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded (4281084974f64 as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded (4293388263f64 as u32) }
                                ) }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#border . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         (if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("1"))) {
                             ({
                                 ({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                                ) . apply_pin (_self) . r#cut ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1)) }
                            ) ;
                             }
                         else {
                             if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("2"))) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                                    ) . apply_pin (_self) . r#copy ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1)) }
                                ) ;
                                 }
                             else {
                                 if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("3"))) {
                                     ({
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                                        ) . apply_pin (_self) . r#paste ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1)) }
                                    ) ;
                                     }
                                 else {
                                     if (((args . 0 . clone ()) . r#id) == (sp :: SharedString :: from ("4"))) {
                                         ({
                                             ({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                                            ) . apply_pin (_self) . r#select_all ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 11u32 - 1)) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_115 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let entries = ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_contextmenuinternal_58_entries }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                ) . apply_pin (_self) . set (entries) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                                        ) . apply_pin (_self) . sub_menu . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_activated }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                                        ) . apply_pin (_self) . activated . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                                ) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                                    ) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             if let Some (current_id) = ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                            ) . apply_pin (_self) . popup_id . take () {
                                 sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                 }
                             let id = sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , true ,) ;
                             ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                            ) . apply_pin (_self) . popup_id . set (Some (id)) ;
                             InnerPopupMenuImpl_root_115 :: user_init (popup_instance_vrc) ;
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#sub_menu) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! []))) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_text_input_62_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . get ()) == (sp :: r#TextWrap :: r#NoWrap)) {
                         ((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_text_input_62_preferred_width }
                        ) . apply_pin (_self) . get () . get () as sp :: Coord))) as _ }
                     else {
                         ({
                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                        ) . apply_pin (_self) . get () . get () }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max (((((- (args . 0 . clone ()) . r#x) as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64)) {
                                     ({
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             ;
                             if (((((args . 0 . clone ()) . r#y as f64) + (({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) < (12f64 as f64)) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                                    ) . apply_pin (_self) . get () . get () as f64) - (({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max (((((- (args . 0 . clone ()) . r#y) as f64) + (12f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#y as f64) + (({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                                ) . apply_pin (_self) . get () . get () as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as f64)) {
                                     ({
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord) . max ((((((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#y as f64)) as f64) - (12f64 as f64)) as f64) - (20f64 as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . window_item () . unwrap () . as_pin_ref () . default_font_size () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 ((400f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_key_pressed }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#key_released) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_key_released }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#page_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#selection_background . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () . color ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) . get () . color ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#TextWrap :: r#WordWrap) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_64 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) . with_alpha (0.2f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#border_65 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) . with_alpha (0.2f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_opacity_66 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4280032286f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                    ) . apply_pin (_self) . get () . get () as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max (((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_touch_area_68_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if false {
                                         (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     as f64)) as sp :: Coord)) as sp :: Coord))) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_touch_area_68_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((false) && (! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if (! (((! false)) && (! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ({
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                        ) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,) }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 ((r#returned_expression0 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression0 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_70 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) . with_alpha (0.2f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#border_71 }
                 + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                        ) . with_alpha (0.2f64 as f32)) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_opacity_72 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (0.6f64) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4280032286f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4294111991f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                    ) . apply_pin (_self) . get () . get () as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max (((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_touch_area_74_pressed_value }
                                    ) . apply_pin (_self) . get () . get () as f64) + (if true {
                                         (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_width }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                                        ) . apply_pin (_self) . get () . get () as f64) / (((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                                        ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                     as f64)) as sp :: Coord)) as sp :: Coord))) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (sp :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_touch_area_74_pressed_value }
                                    ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((true) && (! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                             * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_x as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if (! (((! true)) && (! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ({
                                             {
                                                 }
                                             ;
                                             (true , sp :: r#EventResult :: r#Reject ,) }
                                        ) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max (((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as f64) + ((args . 0 . clone ()) . r#delta_y as f64)) as sp :: Coord)) as sp :: Coord)) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,) }
                                 }
                             ;
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 ((r#returned_expression1 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression1 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground_secondary . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 ((400f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_placeholder_75_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_placeholder_75_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set ({
                 (sp :: r#TextOverflow :: r#Elide) as sp :: r#TextOverflow }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from ("")))) && (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (sp :: SharedString :: from (""))))) {
                         (sp :: SharedString :: from ("")) as _ }
                     else {
                         sp :: SharedString :: from ("") }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Top) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
             + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_style) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#stroke_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
             + sp :: r#ComplexText :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 {
                     }
                 ;
                 {
                     }
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) - (((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 2u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
                 + sp :: r#ComplexText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord , 8f64 as sp :: Coord , 8f64 as sp :: Coord ,) , 5u32 => (((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) + (6f64 as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 6u32 => (((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord , ((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as sp :: Coord , 8f64 as sp :: Coord , 8f64 as sp :: Coord ,) , 7u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_flickable_60_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , 8u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 9u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 10u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 11u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 12u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord , 0f64 as sp :: Coord ,) , 13u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0.8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 14u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                ) . apply_pin (_self) . get () . get () as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((2f64 as f64) + (((((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_height }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) * ((((- ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) / (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_maximum }
                ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) as sp :: Coord ,) , 15u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_vertical_bar_64_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 16u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_67_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 17u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
                ) . apply_pin (_self) . get () . get () as f64) - (16f64 as f64)) as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                ) . apply_pin (_self) . get () . get () as f64)) as sp :: Coord ,) , 18u32 => (0.8f64 as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 19u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((2f64 as f64) + (((((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                ) . apply_pin (_self) . get () . get () as f64) - (4f64 as f64)) as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) * ((((- ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) / (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_maximum }
                ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) as sp :: Coord , ((((({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                ) . apply_pin (_self) . get () . get () as f64) - (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 20u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_horizontal_bar_70_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 21u32 => (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_thumb_73_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#TextInput , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (0u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFontSizeWindow_root_76 {
         r#root_76 : sp :: r#Empty , r#_visibility_77 : sp :: r#Clip , r#rectangle_78 : sp :: r#Empty , r#lineedit_80 : InnerLineEdit_root_37 , r#root_76_empty_79_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_76_empty_79_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_76_empty_79_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_76_font_size : sp :: Property < i32 > , r#root_76_height : sp :: Property < sp :: LogicalLength > , r#root_76_is_visible : sp :: Property < bool > , r#root_76_width : sp :: Property < sp :: LogicalLength > , r#root_76_x : sp :: Property < sp :: LogicalLength > , r#root_76_y : sp :: Property < sp :: LogicalLength > , r#root_76_apply_font_size : sp :: Callback < (f32 ,) , () > , r#root_76_show : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFontSizeWindow_root_76 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFontSizeWindow_root_76 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerLineEdit_root_37 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#lineedit_80 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 3u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                                 + {
                                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                                         InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                                     + {
                                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 50f64 as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                             + {
                                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (160f64 as sp :: Coord) . max (((({
                                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                                 + {
                                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                             + {
                                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (22f64 as sp :: Coord) . max (((({
                                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                                 + {
                                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_layout_42_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_font_size }
            ) . apply_pin (_self) . set ({
                 ((20f64 as i32)) as i32 }
            ) ;
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_is_visible }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_is_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28_accepted }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_apply_font_size }
                            ) . apply_pin (_self) . call (& (({
                                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                             + {
                                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                             + {
                                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () . as_str () . parse :: < f64 > () . unwrap_or_default () as _ ,)) ;
                             ({
                                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_is_visible }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: shared_string_from_number ((({
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_font_size }
                    ) . apply_pin (_self) . get ()) as f64)) as _ }
                ) ;
                 }
             ({
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (56f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerLineEdit_root_37 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#lineedit_80 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_h }
                ) . apply_pin (_self) . get ())))) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_v }
                ) . apply_pin (_self) . get ())))) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (50f64 as sp :: Coord , 80f64 as sp :: Coord , ((((({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                ) . apply_pin (_self) . get () . get () as f64) - (80f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                ) . apply_pin (_self) . get () . get () as f64) - (50f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 3u32 => (({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 56f64 as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 4u32 ..= 12u32 => return {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#TextInput , 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (3u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (3u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (3u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                 + {
                     InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
                 + {
                     * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (3u32 , sp :: AccessibilityAction :: r#SetValue (args)) => {
                     let args = (args ,) ;
                     ({
                         InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
                     + {
                         * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37_accessible_action_set_value }
                    ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                 (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (4u32 ..= 12u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . accessibility_action (index - 4u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: SupportedAccessibilityAction :: r#SetValue , 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 4u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 4u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 ..= 12u32 => {
                     * & Self :: FIELD_OFFSETS . r#lineedit_80 }
                 . apply_pin (_self) . item_element_infos (index - 4u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItemBase_root_81 {
         r#root_81 : sp :: r#Empty , r#background_layer_82 : sp :: r#BasicBorderRectangle , r#touch_area_83 : sp :: r#TouchArea , r#label_85 : sp :: r#SimpleText , r#root_81_background_layer_82_height : sp :: Property < sp :: LogicalLength > , r#root_81_current_background : sp :: Property < slint :: Brush > , r#root_81_current_foreground : sp :: Property < slint :: Brush > , r#root_81_default_foreground : sp :: Property < slint :: Brush > , r#root_81_entry : sp :: Property < sp :: MenuEntry > , r#root_81_height : sp :: Property < sp :: LogicalLength > , r#root_81_icon_size : sp :: Property < sp :: LogicalLength > , r#root_81_is_current : sp :: Property < bool > , r#root_81_layout_84_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_81_layout_84_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_81_layout_84_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_81_padding_left : sp :: Property < sp :: LogicalLength > , r#root_81_padding_right : sp :: Property < sp :: LogicalLength > , r#root_81_sub_menu_icon : sp :: Property < sp :: Image > , r#root_81_touch_area_83_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_81_touch_area_83_width : sp :: Property < sp :: LogicalLength > , r#root_81_width : sp :: Property < sp :: LogicalLength > , r#root_81_x : sp :: Property < sp :: LogicalLength > , r#root_81_activate : sp :: Callback < (sp :: MenuEntry , sp :: Coord ,) , () > , r#root_81_clear_current : sp :: Callback < () , () > , r#root_81_set_current : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_image_86 > , change_tracker0 : sp :: ChangeTracker , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItemBase_root_81 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItemBase_root_81 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ((({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                    ) . apply_pin (_self) . get ()) . r#has_sub_menu as bool)) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_background_layer_82_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = ({
                                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_left }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#end = ({
                                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_right }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_touch_area_83_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = ({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_left }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct . r#end = ({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_right }
                            ) . apply_pin (_self) . get () . get () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ) ;
                         InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_touch_area_83_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = (((r#parent_position . clone ()) . r#x as f64) + (0f64 as f64)) as _ ;
                             the_struct . r#y = (((r#parent_position . clone ()) . r#y as f64) + (0f64 as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_touch_area_83_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#background_layer_82 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_is_current }
                    ) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (({
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_current_background }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Move))) && ((! ({
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_is_current }
                            ) . apply_pin (_self) . get ()))) {
                                 ({
                                     ({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_set_current }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Down))) && ((({
                                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                                ) . apply_pin (_self) . get ()) . r#has_sub_menu)) {
                                     ({
                                         ({
                                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_activate }
                                        ) . apply_pin (_self) . call (& (({
                                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                                        ) . apply_pin (_self) . get () as _ , (({
                                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_touch_area_83_absolute_position }
                                        ) . apply_pin (_self) . get ()) . r#y as _ ,)) }
                                    ) ;
                                     }
                                 else {
                                     if (((((((((((args . 0 . clone ()) . r#kind) == (sp :: r#PointerEventKind :: r#Up))) && (((({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) > (0f64 as f64))))) && (((({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) < (({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                                    ) . apply_pin (_self) . get () . get () as f64))))) && (((({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) > (0f64 as f64))))) && (((({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) < (({
                                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                                    ) . apply_pin (_self) . get () . get () as f64)))) {
                                         ({
                                             ({
                                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_activate }
                                            ) . apply_pin (_self) . call (& (({
                                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                                            ) . apply_pin (_self) . get () as _ , (({
                                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_touch_area_83_absolute_position }
                                            ) . apply_pin (_self) . get ()) . r#y as _ ,)) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (if ({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_is_current }
                    ) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         0f64 }
                     as f64) , & (1f64 as f64)) {
                         (({
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_current_foreground }
                        ) . apply_pin (_self) . get ()) as _ }
                     else {
                         ({
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_default_foreground }
                        ) . apply_pin (_self) . get () }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_background_layer_82_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (0f64 as f64)) as f64) - (0f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                    ) . apply_pin (_self) . get ()) . r#title) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#background_layer_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#background_layer_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     if (((! ({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) && (({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_is_current }
                    ) . apply_pin (_self) . get ())) {
                         (({
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_clear_current }
                        ) . apply_pin (_self) . call (& ())) ;
                         }
                     else {
                         {
                             }
                         }
                     }
                 ;
                 }
            ) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_h }
                ) . apply_pin (_self) . get ())))))) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ((({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                ) . apply_pin (_self) . get ())))))) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMenuItemBase_root_81 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_86 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (((((({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                ) . apply_pin (_self) . get () . get () as f64) - (0f64 as f64)) as f64) - (0f64 as f64)) as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((({
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                ) . apply_pin (_self) . get ()) . r#title) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_86 {
         r#image_86 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_86 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_81 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_86 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                     + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_background_layer_82_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (0f64 as f64)) as f64) - (0f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_sub_menu_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (0f64 as f64)) as f64) - (0f64 as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_86 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_81 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMenuItemBase_root_81 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_86 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_86 :: FIELD_OFFSETS . r#image_86 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_86) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_86 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_image_86 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_86 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_86 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_86 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_86 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerMenuItem_root_88 {
         r#root_88 : sp :: r#Empty , r#base_90 : InnerMenuItemBase_root_81 , r#root_88_empty_89_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_88_empty_89_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_88_empty_89_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_88_height : sp :: Property < sp :: LogicalLength > , r#root_88_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_88_width : sp :: Property < sp :: LogicalLength > , r#root_88_x : sp :: Property < sp :: LogicalLength > , r#root_88_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMenuItem_root_88 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMenuItem_root_88 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItemBase_root_81 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_90 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 2u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                             + {
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_h }
                            ) . apply_pin (_self) . get ())))))) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                         + {
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_h }
                        ) . apply_pin (_self) . get ())))))) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                         + {
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                        ) . apply_pin (_self) . get ())))))) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let r#layout_info_6 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_6 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_6 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (22f64 as sp :: Coord) . max ((((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                             + {
                                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                            ) . apply_pin (_self) . get ()))))))) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_6 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_6 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_6 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    ) + (({
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#background_layer_82 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_current_background }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32))) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4281494735f64 as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_current_foreground }
            ) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4293388263f64 as u32))) as slint :: Brush }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_default_foreground }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4294111991f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4280032286f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((0.9996999999999999f64 as f64) * (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . window_item () . unwrap () . as_pin_ref () . default_font_size () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 ((400f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (13f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_left }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_right }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_sub_menu_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_current_foreground }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_left }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_padding_right }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_sub_menu_icon }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItemBase_root_81 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_90 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_90 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (22f64 as sp :: Coord) . max ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                         + {
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                        ) . apply_pin (_self) . get ()))))))) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_90 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_90 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 ..= 5u32 => return {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . item_geometry (index - 2u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessible_role (0) , 2u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessible_role (index - 2u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (2u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessible_string_property (index - 2u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (2u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . accessibility_action (index - 2u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 2u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 2u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_90 }
                 . apply_pin (_self) . item_element_infos (index - 2u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerPopupMenuImpl_root_115 {
         r#root_115 : sp :: r#WindowItem , r#focus_scope_117 : sp :: r#FocusScope , r#frame_shadow_118 : sp :: r#BoxShadow , r#frame_119 : sp :: r#BasicBorderRectangle , r#frame_clip_120 : sp :: r#Clip , r#sub_menu_124 : sp :: r#ContextMenu , r#root_115_current : sp :: Property < i32 > , r#root_115_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_115_frame_119_height : sp :: Property < sp :: LogicalLength > , r#root_115_layout_121_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_115_layout_121_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_115_layout_121_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_115_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_115_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_115_optimized_open_sub_menu_after_timeout_116_interval : sp :: Property < i64 > , r#root_115_optimized_open_sub_menu_after_timeout_116_running : sp :: Property < bool > , r#root_115_sub_menu_124_absolute_position : sp :: Property < slint :: LogicalPosition > , r#root_115_sub_menu_124_entries : sp :: Property < sp :: ModelRc < sp :: MenuEntry > > , r#root_115_activated : sp :: Callback < (sp :: MenuEntry ,) , () > , r#root_115_close : sp :: Callback < () , () > , r#root_115_optimized_open_sub_menu_after_timeout_116_triggered : sp :: Callback < () , () > , r#root_115_sub_menu : sp :: Callback < (sp :: MenuEntry ,) , sp :: ModelRc < sp :: MenuEntry > > , repeater0 : sp :: Repeater < InnerComponent_menuitem_122 > , change_tracker0 : sp :: ChangeTracker , change_tracker1 : sp :: ChangeTracker , timer0 : sp :: Timer , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_115 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerPopupMenuImpl_root_115 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
            ) . apply_pin (_self) . set ({
                 ((- 1f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: MenuEntry > :: from (sp :: vec ! []))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_frame_119_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 4f64 as _ ;
                                 the_struct . r#end = 4f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 4f64 as _ ;
                             the_struct . r#end = 4f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 4f64 as _ ;
                             the_struct . r#end = 4f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let r#layout_info_7 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_7 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_7 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (280f64 as sp :: Coord) . max (((({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                             the_struct . r#min_percent = (r#layout_info_7 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_7 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_7 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    ) + (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_interval }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (500f64) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_triggered }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
                            ) . apply_pin (_self) . set (false as _) ;
                             if ((({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                            ) . apply_pin (_self) . get () as f64) >= ((0f64 as i32) as f64)) {
                                 ({
                                     if (match & ({
                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                    ) . apply_pin (_self) . get () {
                                         x => {
                                             let index = (({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                            ) . apply_pin (_self) . get ()) as usize ;
                                             x . row_data_tracked (index) . unwrap_or_default () }
                                         }
                                    ) . r#has_sub_menu {
                                         ({
                                             _self . r#fn_activate (match & ({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     let index = (({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . get ()) as usize ;
                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                 }
                                             as _ , _self . r#fn_focus_scope_117_y_pos (({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                            ) . apply_pin (_self) . get () as _) as _) }
                                        ) ;
                                         }
                                     else {
                                         {
                                             ({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                                            ) . apply_pin (_self) . r#close ((& _self . globals . get () . unwrap () . window_adapter_impl ()) , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) }
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu_124_absolute_position }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#parent_position = sp :: logical_position_to_api ((* & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) . map_to_window (:: core :: default :: Default :: default ())) ;
                         ;
                         {
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = (((r#parent_position . clone ()) . r#x as f64) + (0f64 as f64)) as _ ;
                             the_struct . r#y = (((r#parent_position . clone ()) . r#y as f64) + (0f64 as f64)) as _ ;
                             the_struct }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#focus_scope_117 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#focus_scope_117 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression2 = {
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
                                ) . apply_pin (_self) . set (false as _) ;
                                 let r#return_check_merge2 = if (((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\u{f700}"))) {
                                     ((false , {
                                         if ((({
                                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                        ) . apply_pin (_self) . get () as f64) >= ((1f64 as i32) as f64)) {
                                             ({
                                                 ({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                ) . apply_pin (_self) . set (((({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                ) . apply_pin (_self) . get () as f64) - ((1f64 as i32) as f64)) as _) }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 ({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                ) . apply_pin (_self) . set ((((match & ({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                 as f64) - (1f64 as f64)) as i32) as _) }
                                             }
                                         ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if (((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\u{f701}"))) {
                                         ((false , {
                                             if ((({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                            ) . apply_pin (_self) . get () as f64) < ((((match & ({
                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     x . model_tracker () . track_row_count_changes () ;
                                                     x . row_count () as i32 }
                                                 }
                                             as f64) - (1f64 as f64)) as i32) as f64)) {
                                                 ({
                                                     ({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . set (((({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . get () as f64) + ((1f64 as i32) as f64)) as _) }
                                                ) ;
                                                 }
                                             else {
                                                 {
                                                     ({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . set ((0f64 as i32) as _) }
                                                 }
                                             ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,)) as _ }
                                     else {
                                         if (((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n"))) {
                                             ((false , {
                                                 if ((((({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                ) . apply_pin (_self) . get () as f64) >= ((0f64 as i32) as f64))) && (((({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                ) . apply_pin (_self) . get () as f64) < (match & ({
                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                ) . apply_pin (_self) . get () {
                                                     x => {
                                                         x . model_tracker () . track_row_count_changes () ;
                                                         x . row_count () as i32 }
                                                     }
                                                 as f64)))) {
                                                     ({
                                                         _self . r#fn_activate (match & ({
                                                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                        ) . apply_pin (_self) . get () {
                                                             x => {
                                                                 let index = (({
                                                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                                ) . apply_pin (_self) . get ()) as usize ;
                                                                 x . row_data_tracked (index) . unwrap_or_default () }
                                                             }
                                                         as _ , _self . r#fn_focus_scope_117_y_pos (({
                                                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                        ) . apply_pin (_self) . get () as _) as _) }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         }
                                                     }
                                                 ;
                                                 sp :: r#EventResult :: r#Accept }
                                             ,)) as _ }
                                         else {
                                             if (! (((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\u{f703}")))) {
                                                 ({
                                                     if (((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\u{f702}"))) {
                                                         ({
                                                             ({
                                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                                                            ) . apply_pin (_self) . call (& ()) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     (true , sp :: r#EventResult :: r#Reject ,) }
                                                ) as _ }
                                             else {
                                                 (false , {
                                                     if ((((((({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . get () as f64) >= ((0f64 as i32) as f64))) && (((({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                    ) . apply_pin (_self) . get () as f64) < (match & ({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                    ) . apply_pin (_self) . get () {
                                                         x => {
                                                             x . model_tracker () . track_row_count_changes () ;
                                                             x . row_count () as i32 }
                                                         }
                                                     as f64))))) && ((match & ({
                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                    ) . apply_pin (_self) . get () {
                                                         x => {
                                                             let index = (({
                                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                            ) . apply_pin (_self) . get ()) as usize ;
                                                             x . row_data_tracked (index) . unwrap_or_default () }
                                                         }
                                                    ) . r#has_sub_menu)) {
                                                         ({
                                                             _self . r#fn_activate (match & ({
                                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                                            ) . apply_pin (_self) . get () {
                                                                 x => {
                                                                     let index = (({
                                                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                                    ) . apply_pin (_self) . get ()) as usize ;
                                                                     x . row_data_tracked (index) . unwrap_or_default () }
                                                                 }
                                                             as _ , _self . r#fn_focus_scope_117_y_pos (({
                                                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current }
                                                            ) . apply_pin (_self) . get () as _) as _) }
                                                        ) ;
                                                         }
                                                     else {
                                                         {
                                                             }
                                                         }
                                                     ;
                                                     sp :: r#EventResult :: r#Accept }
                                                 ,) }
                                             }
                                         }
                                     }
                                 ;
                                 ;
                                 if (r#return_check_merge2 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge2 . clone ()) . 1 ,) }
                                 }
                             ;
                             ;
                             if (r#returned_expression2 . clone ()) . 1 {
                                 ((r#returned_expression2 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression2 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (22f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (sp :: Color :: from_argb_encoded (1711276032f64 as u32)) as sp :: Color }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (0.5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded (4280690214f64 as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded (4293980400f64 as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#border . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (6f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#activated) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_activated }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) ;
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let position = args . 0 . clone () ;
                             let popup_instance = InnerPopupMenuImpl_root_115 :: new (_self . globals . get () . unwrap () . clone ()) . unwrap () ;
                             let popup_instance_vrc = sp :: VRc :: map (popup_instance . clone () , | x | x) ;
                             let parent_weak = _self . self_weak . get () . unwrap () . clone () ;
                             let entries = ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu_124_entries }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let _self = popup_instance_vrc . as_pin_ref () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
                                ) . apply_pin (_self) . set (entries) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                                        ) . apply_pin (_self) . sub_menu . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_activated }
                                ) . apply_pin (_self) . set_handler (move | entry | {
                                     if let Some (self_rc) = self_weak . upgrade () {
                                         let _self = self_rc . as_pin_ref () ;
                                         ({
                                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                                        ) . apply_pin (_self) . activated . call (entry) }
                                     else {
                                         Default :: default () }
                                     }
                                ) ;
                                 let self_weak = parent_weak . clone () ;
                                 ({
                                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                                ) . apply_pin (_self) . set_handler (move | () | {
                                     let Some (self_rc) = self_weak . upgrade () else {
                                         return }
                                     ;
                                     let _self = self_rc . as_pin_ref () ;
                                     if let Some (current_id) = ({
                                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                                    ) . apply_pin (_self) . popup_id . take () {
                                         sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                         }
                                     }
                                ) ;
                                 }
                             if let Some (current_id) = ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                            ) . apply_pin (_self) . popup_id . take () {
                                 sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . close_popup (current_id) ;
                                 }
                             let id = sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . show_popup (& sp :: VRc :: into_dyn (popup_instance . into ()) , position , sp :: PopupClosePolicy :: CloseOnClickOutside , & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1) , true ,) ;
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                            ) . apply_pin (_self) . popup_id . set (Some (id)) ;
                             InnerPopupMenuImpl_root_115 :: user_init (popup_instance_vrc) ;
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                 + sp :: r#ContextMenu :: FIELD_OFFSETS . r#sub_menu) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu }
                            ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#focus_scope_117 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#blur) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
             + sp :: r#BoxShadow :: FIELD_OFFSETS . r#offset_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . r#fn_focus () ;
             {
                 }
             ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker0 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_interval }
                ) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
             # [allow (dead_code , unused)] _self . change_tracker1 . init (self_weak , move | self_weak | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
                ) . apply_pin (_self) . get () }
             , move | self_weak , _ | {
                 let self_rc = self_weak . upgrade () . unwrap () ;
                 let _self = self_rc . as_pin_ref () ;
                 {
                     _self . update_timers () }
                 ;
                 }
            ) ;
             _self . update_timers () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_menuitem_122 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         fn update_timers (self : :: core :: pin :: Pin < & Self >) {
             let _self = self ;
             if ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
            ) . apply_pin (_self) . get () {
                 let interval = core :: time :: Duration :: from_millis (({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_interval }
                ) . apply_pin (_self) . get () as u64) ;
                 if ! self . timer0 . running () || interval != self . timer0 . interval () {
                     let self_weak = self . self_weak . get () . unwrap () . clone () ;
                     self . timer0 . start (sp :: TimerMode :: Repeated , interval , move || {
                         if let Some (self_rc) = self_weak . upgrade () {
                             let _self = self_rc . as_pin_ref () ;
                             ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_triggered }
                            ) . apply_pin (_self) . call (& ()) }
                         }
                    ) ;
                     }
                 }
             else {
                 self . timer0 . stop () ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_activate (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: MenuEntry , arg_1 : sp :: Coord ,) -> () {
             let _self = self ;
             let args = (arg_0 , arg_1 ,) ;
             ({
                 ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running }
                ) . apply_pin (_self) . set (false as _) ;
                 if (args . 0 . clone ()) . r#has_sub_menu {
                     ({
                         ({
                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu_124_entries }
                        ) . apply_pin (_self) . set (({
                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu }
                        ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) as _) ;
                         ({
                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
                         + sp :: r#ContextMenu :: FIELD_OFFSETS . r#show) . apply_pin (_self) . call (& ({
                             let mut the_struct = slint :: LogicalPosition :: default () ;
                             the_struct . r#x = ({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ ;
                             the_struct . r#y = ((args . 1 . clone () as f64) - ((({
                                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_sub_menu_124_absolute_position }
                            ) . apply_pin (_self) . get ()) . r#y as f64)) as _ ;
                             the_struct }
                         as _ ,)) }
                    ) ;
                     }
                 else {
                     {
                         ({
                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_activated }
                        ) . apply_pin (_self) . call (& (args . 0 . clone () as _ ,)) ;
                         ({
                             * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_close }
                        ) . apply_pin (_self) . call (& ()) }
                     }
                 }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1) , true)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus_scope_117_y_pos (self : :: core :: pin :: Pin < & Self > , arg_0 : i32 ,) -> sp :: Coord {
             let _self = self ;
             let args = (arg_0 ,) ;
             (((4f64 as f64) + (((((args . 0 . clone () as f64) * (((({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_frame_119_height }
            ) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64)) as f64) / (match & ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_entries }
            ) . apply_pin (_self) . get () {
                 x => {
                     x . model_tracker () . track_row_count_changes () ;
                     x . row_count () as i32 }
                 }
             as f64)) as f64))) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_menuitem_122 {
         r#menuitem_122 : InnerMenuItem_root_88 , r#model_data : sp :: Property < sp :: MenuEntry > , r#model_index : sp :: Property < i32 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_menuitem_122 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_115 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_menuitem_122 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerMenuItem_root_88 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#menuitem_122 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_activate }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = _self . parent . upgrade () . as_ref () . map (| x | x . as_pin_ref () . r#fn_activate (args . 0 . clone () as _ , args . 1 . clone () as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_clear_current }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (((- 1f64) as i32) as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_entry }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_is_current }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ApproxEq :: < f64 > :: approx_eq (& ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () as f64) , & (({
                         * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64))) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                 + {
                     * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_set_current }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             sp :: WindowInner :: from_pub ((& _self . globals . get () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . parent . upgrade () . unwrap () . as_pin_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . parent . upgrade () . unwrap () . as_pin_ref () . tree_index_of_first_child . get () + 1u32 - 1) , true) ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_current) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (({
                                     * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
                                ) . apply_pin (_self) . get () as _)) ;
                                 }
                             ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_optimized_open_sub_menu_after_timeout_116_running) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (true as _)) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((_self . parent . upgrade () . as_ref () . map (| x | ({
                         * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115_layout_121_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerMenuItem_root_88 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#menuitem_122 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_empty_89_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                     + {
                         * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (22f64 as sp :: Coord) . max ((((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                         + {
                             InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
                         + {
                             * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81_layout_84_layoutinfo_v }
                        ) . apply_pin (_self) . get ()))))))) . r#min as sp :: Coord)) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 0u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((_self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as f64) - (4f64 as f64)) as f64) - (4f64 as f64)) as sp :: Coord , 4f64 as sp :: Coord , ({
                     InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
                 + {
                     * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 ..= 5u32 => return {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#menuitem_122 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_menuitem_122 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_115 > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerPopupMenuImpl_root_115 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             6usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 3u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 4u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 6u32 , parent_index : 3u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 3u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_menuitem_122 , sp :: ItemVTable , sp :: AllowPin > ;
             5usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 * & InnerMenuItem_root_88 :: FIELD_OFFSETS . r#root_88 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#root_81 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#background_layer_82 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#touch_area_83 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#menuitem_122 }
             + {
                 InnerMenuItem_root_88 :: FIELD_OFFSETS . r#base_90 }
             + {
                 * & InnerMenuItemBase_root_81 :: FIELD_OFFSETS . r#label_85 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_menuitem_122) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_menuitem_122 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_menuitem_122 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_menuitem_122 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_menuitem_122 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_menuitem_122 {
         type Data = sp :: MenuEntry ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_menuitem_122 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_menuitem_122 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerPopupMenuImpl_root_115 {
         fn new (globals : sp :: Rc < SharedGlobals >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = globals ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             7usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 5u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerPopupMenuImpl_root_115 , sp :: ItemVTable , sp :: AllowPin > ;
             6usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#root_115 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#focus_scope_117 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#sub_menu_124 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_shadow_118 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_119 }
            ) , sp :: VOffset :: new ({
                 * & InnerPopupMenuImpl_root_115 :: FIELD_OFFSETS . r#frame_clip_120 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerPopupMenuImpl_root_115) ;
         }
     ;
     impl sp :: PinnedDrop for InnerPopupMenuImpl_root_115 {
         fn drop (self : core :: pin :: Pin < & mut InnerPopupMenuImpl_root_115 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerPopupMenuImpl_root_115 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerPopupMenuImpl_root_115 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerTabDemo {
         r#root_91 : sp :: r#WindowItem , r#empty_93 : sp :: r#Empty , r#empty_94 : sp :: r#Empty , r#rectangle_98 : sp :: r#Empty , r#empty_99 : sp :: r#Empty , r#rectangle_105 : sp :: r#Empty , r#_clip_106 : sp :: r#Clip , r#editor_area_visibility_110 : sp :: r#Clip , r#styled_text_area_visibility_112 : sp :: r#Clip , r#styled_text_area_113 : sp :: r#SimpleText , r#button_95 : InnerButton_root_1 , r#button_96 : InnerButton_root_1 , r#button_97 : InnerButton_root_1 , r#button_100 : InnerButton_root_1 , r#button_101 : InnerButton_root_1 , r#button_102 : InnerButton_root_1 , r#button_103 : InnerButton_root_1 , r#button_104 : InnerButton_root_1 , r#editor_area_111 : InnerTextEdit_root_50 , r#font_window_114 : InnerFontSizeWindow_root_76 , r#root_91_current_tab : sp :: Property < i32 > , r#root_91_empty_92_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_91_empty_92_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_92_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_93_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_91_empty_93_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_93_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_94_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_91_empty_94_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_94_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_99_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_91_empty_99_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_empty_99_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_font_size : sp :: Property < i32 > , r#root_91_is_bold : sp :: Property < bool > , r#root_91_is_editing : sp :: Property < bool > , r#root_91_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_tab_bar_107_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_91_tab_bar_107_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_91_tab_bar_107_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_91_tab_contents : sp :: Property < sp :: ModelRc < sp :: SharedString > > , r#root_91_tab_titles : sp :: Property < sp :: ModelRc < sp :: SharedString > > , r#root_91_add_new_tab : sp :: Callback < () , () > , r#root_91_apply_bold : sp :: Callback < () , () > , r#root_91_apply_font_size : sp :: Callback < (f32 ,) , () > , r#root_91_apply_underline : sp :: Callback < () , () > , r#root_91_close_tab : sp :: Callback < () , () > , r#root_91_current_tab_changed : sp :: Callback < (i32 ,) , () > , r#root_91_rename_tab : sp :: Callback < (i32 , sp :: SharedString ,) , () > , r#root_91_update_tab_content : sp :: Callback < (sp :: SharedString ,) , () > , repeater0 : sp :: Repeater < InnerComponent_tabcomponent_108 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerTabDemo >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerTabDemo {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_95 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 9u32 - 1 , tree_index_of_first_child + 12u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_96 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 10u32 - 1 , tree_index_of_first_child + 20u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_97 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 11u32 - 1 , tree_index_of_first_child + 28u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_100 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 36u32 - 1 , tree_index_of_first_child + 41u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_101 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 37u32 - 1 , tree_index_of_first_child + 49u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_102 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 38u32 - 1 , tree_index_of_first_child + 57u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_103 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 39u32 - 1 , tree_index_of_first_child + 65u32 - 1) ;
             InnerButton_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_104 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 40u32 - 1 , tree_index_of_first_child + 73u32 - 1) ;
             InnerTextEdit_root_50 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#editor_area_111 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 83u32 - 1 , tree_index_of_first_child + 84u32 - 1) ;
             InnerFontSizeWindow_root_76 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#font_window_114 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 5u32 - 1 , tree_index_of_first_child + 106u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#background . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab }
            ) . apply_pin (_self) . set ({
                 ((0f64 as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 40f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 40f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 55f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 55f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                                 + {
                                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _ , r#size : ((1f64 as f64) * (600f64 as f64)) as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 780f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 780f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 780f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 780f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                             + {
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 800f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 800f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 800f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 800f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 5f64 as _ ;
                         the_struct . r#end = 5f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 40f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 40f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 55f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 55f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                             + {
                                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 5f64 as _ ;
                         the_struct . r#end = 5f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerTabDemo :: FIELD_OFFSETS . r#rectangle_98 }
                                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : 780f64 as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#rectangle_98 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#rectangle_98 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 70f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 70f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 70f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 70f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 80f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 80f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 70f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 70f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 70f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 70f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 80f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 80f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 70f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 70f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 100f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 100f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 70f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 70f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 60f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 60f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                                 + {
                                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 60f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 60f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 70f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 70f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 100f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 100f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 70f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 70f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 60f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 60f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 60f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 60f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                             + {
                                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 30f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 30f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
            ) . apply_pin (_self) . set ({
                 ((20f64 as i32)) as i32 }
            ) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (600f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_bold }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . globals . get () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : 780f64 as _ , r#spacing : 20f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 20f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_contents }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: SharedString > :: from (sp :: vec ! [sp :: SharedString :: from ("") as _]))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (sp :: VecModel :: < sp :: SharedString > :: from (sp :: vec ! [sp :: SharedString :: from ("Tab 1") as _]))) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (800f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                             + {
                                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_show }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Font Size")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (70f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_bold }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Bold")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (70f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_underline }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Underline")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (80f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_add_new_tab }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Add Tab")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (70f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_rename_tab }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab }
                            ) . apply_pin (_self) . get () as _ , match & ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles }
                            ) . apply_pin (_self) . get () {
                                 x => {
                                     let index = (({
                                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab }
                                    ) . apply_pin (_self) . get ()) as usize ;
                                     x . row_data_tracked (index) . unwrap_or_default () }
                                 }
                             as _ ,)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Rename Tab")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (100f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_close_tab }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Close Tab")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (70f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
                            ) . apply_pin (_self) . set (false as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Save")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (60f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (30f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Copy All")) as sp :: SharedString }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (60f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                    ) . apply_pin (_self) . get () [8usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
                    ) . apply_pin (_self) . get () as f64) * (1f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: SharedString :: from ("")) as _ }
                ) ;
                 }
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (800f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (5f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! (! ({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerCupertinoPalette_127 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_CupertinoPalette_127 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
                    ) . apply_pin (_self) . get () as f64) * (1f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((if ({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_bold }
                    ) . apply_pin (_self) . get () {
                         (700f64) as _ }
                     else {
                         400f64 }
                     as i32)) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                     + {
                         * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                     + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (800f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_apply_font_size }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
                            ) . apply_pin (_self) . set ((args . 0 . clone () as i32) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_font_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))))) . r#preferred as sp :: Coord) . max ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))))) . r#min as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_show }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                             + {
                                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_is_visible }
                            ) . apply_pin (_self) . set (true as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))))) . r#preferred as sp :: Coord) . max ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_empty_79_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))))) . r#min as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((800f64 as f64) - (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((600f64 as f64) - (({
                         InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                     + {
                         * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_95 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_96 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_97 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_100 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_101 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_102 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_103 }
             . apply_pin (x)) ,) ;
             InnerButton_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_104 }
             . apply_pin (x)) ,) ;
             InnerTextEdit_root_50 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#editor_area_111 }
             . apply_pin (x)) ,) ;
             InnerFontSizeWindow_root_76 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#font_window_114 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_95 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 1u32 , order , visitor) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 5u32 , order , visitor) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_97 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 9u32 , order , visitor) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_100 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 13u32 , order , visitor) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_101 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 17u32 , order , visitor) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_102 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 21u32 , order , visitor) }
                 25u32 ..= 28u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_103 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 25u32 , order , visitor) }
                 29u32 ..= 32u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_104 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 29u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 800f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 800f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 600f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 600f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_95 }
                     . apply_pin (_self) . subtree_range (dyn_index - 1u32) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . subtree_range (dyn_index - 5u32) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_97 }
                     . apply_pin (_self) . subtree_range (dyn_index - 9u32) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_100 }
                     . apply_pin (_self) . subtree_range (dyn_index - 13u32) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_101 }
                     . apply_pin (_self) . subtree_range (dyn_index - 17u32) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_102 }
                     . apply_pin (_self) . subtree_range (dyn_index - 21u32) }
                 25u32 ..= 28u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_103 }
                     . apply_pin (_self) . subtree_range (dyn_index - 25u32) }
                 29u32 ..= 32u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_104 }
                     . apply_pin (_self) . subtree_range (dyn_index - 29u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerTabDemo :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_tabcomponent_108 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 ..= 4u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_95 }
                     . apply_pin (_self) . subtree_component (dyn_index - 1u32 , subtree_index , result) }
                 5u32 ..= 8u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_96 }
                     . apply_pin (_self) . subtree_component (dyn_index - 5u32 , subtree_index , result) }
                 9u32 ..= 12u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_97 }
                     . apply_pin (_self) . subtree_component (dyn_index - 9u32 , subtree_index , result) }
                 13u32 ..= 16u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_100 }
                     . apply_pin (_self) . subtree_component (dyn_index - 13u32 , subtree_index , result) }
                 17u32 ..= 20u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_101 }
                     . apply_pin (_self) . subtree_component (dyn_index - 17u32 , subtree_index , result) }
                 21u32 ..= 24u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_102 }
                     . apply_pin (_self) . subtree_component (dyn_index - 21u32 , subtree_index , result) }
                 25u32 ..= 28u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_103 }
                     . apply_pin (_self) . subtree_component (dyn_index - 25u32 , subtree_index , result) }
                 29u32 ..= 32u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_104 }
                     . apply_pin (_self) . subtree_component (dyn_index - 29u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (600f64 as sp :: Coord , 800f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (40f64 as sp :: Coord , 780f64 as sp :: Coord , 5f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (55f64 as sp :: Coord , 780f64 as sp :: Coord , 5f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ((((800f64 as f64) - (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_width }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((600f64 as f64) - (({
                     InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
                 + {
                     * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76_height }
                ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 6u32 => (16f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 7u32 => (16f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 8u32 => (16f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_93_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 9u32 => (30f64 as sp :: Coord , 70f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 10u32 => (30f64 as sp :: Coord , 70f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 11u32 => (30f64 as sp :: Coord , 80f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_94_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 36u32 => (30f64 as sp :: Coord , 70f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 37u32 => (30f64 as sp :: Coord , 100f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 38u32 => (30f64 as sp :: Coord , 70f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 39u32 => (30f64 as sp :: Coord , 60f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 40u32 => (30f64 as sp :: Coord , 60f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_99_layout_cache }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , 12f64 as sp :: Coord ,) , 81u32 => (55f64 as sp :: Coord , 780f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 83u32 => (((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as sp :: Coord , 800f64 as sp :: Coord , 5f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord ,) , 105u32 => (((((1f64 as f64) * (600f64 as f64)) as f64) - (120f64 as f64)) as sp :: Coord , 800f64 as sp :: Coord , 5f64 as sp :: Coord , ({
                     * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_empty_92_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord ,) , 12u32 ..= 19u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . item_geometry (index - 12u32 + 1) , 20u32 ..= 27u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . item_geometry (index - 20u32 + 1) , 28u32 ..= 35u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . item_geometry (index - 28u32 + 1) , 41u32 ..= 48u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . item_geometry (index - 41u32 + 1) , 49u32 ..= 56u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . item_geometry (index - 49u32 + 1) , 57u32 ..= 64u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . item_geometry (index - 57u32 + 1) , 65u32 ..= 72u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . item_geometry (index - 65u32 + 1) , 73u32 ..= 80u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . item_geometry (index - 73u32 + 1) , 84u32 ..= 104u32 => return {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . item_geometry (index - 84u32 + 1) , 106u32 ..= 117u32 => return {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . item_geometry (index - 106u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 => sp :: r#AccessibleRole :: r#Button , 10u32 => sp :: r#AccessibleRole :: r#Button , 11u32 => sp :: r#AccessibleRole :: r#Button , 36u32 => sp :: r#AccessibleRole :: r#Button , 37u32 => sp :: r#AccessibleRole :: r#Button , 38u32 => sp :: r#AccessibleRole :: r#Button , 39u32 => sp :: r#AccessibleRole :: r#Button , 40u32 => sp :: r#AccessibleRole :: r#Button , 83u32 => sp :: r#AccessibleRole :: r#TextInput , 105u32 => sp :: r#AccessibleRole :: r#Text , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessible_role (0) , 12u32 ..= 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessible_role (index - 12u32 + 1) , 10u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_role (0) , 20u32 ..= 27u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_role (index - 20u32 + 1) , 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessible_role (0) , 28u32 ..= 35u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessible_role (index - 28u32 + 1) , 36u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessible_role (0) , 41u32 ..= 48u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessible_role (index - 41u32 + 1) , 37u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessible_role (0) , 49u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessible_role (index - 49u32 + 1) , 38u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessible_role (0) , 57u32 ..= 64u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessible_role (index - 57u32 + 1) , 39u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessible_role (0) , 65u32 ..= 72u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessible_role (index - 65u32 + 1) , 40u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessible_role (0) , 73u32 ..= 80u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessible_role (index - 73u32 + 1) , 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessible_role (0) , 84u32 ..= 104u32 => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessible_role (index - 84u32 + 1) , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessible_role (0) , 106u32 ..= 117u32 => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessible_role (index - 106u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (9u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (9u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (10u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (10u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (11u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (11u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (11u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (11u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (36u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (36u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (36u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (36u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (37u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (37u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (37u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (37u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (38u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (38u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (38u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (38u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (39u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (39u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (39u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (39u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (40u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (40u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (40u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (40u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                 + {
                     * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_text }
                ) . apply_pin (_self) . get ()) , (83u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (83u32 , sp :: AccessibleStringProperty :: r#PlaceholderText) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50_accessible_placeholder_text }
                ) . apply_pin (_self) . get ()) , (83u32 , sp :: AccessibleStringProperty :: r#ReadOnly) => sp :: Some (if ({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (83u32 , sp :: AccessibleStringProperty :: r#Value) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (105u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
                 + {
                     * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
                 + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (12u32 ..= 19u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessible_string_property (index - 12u32 + 1 , what) , (10u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (20u32 ..= 27u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessible_string_property (index - 20u32 + 1 , what) , (11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (28u32 ..= 35u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessible_string_property (index - 28u32 + 1 , what) , (36u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (41u32 ..= 48u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessible_string_property (index - 41u32 + 1 , what) , (37u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (49u32 ..= 56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessible_string_property (index - 49u32 + 1 , what) , (38u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (57u32 ..= 64u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessible_string_property (index - 57u32 + 1 , what) , (39u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (65u32 ..= 72u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessible_string_property (index - 65u32 + 1 , what) , (40u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (73u32 ..= 80u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessible_string_property (index - 73u32 + 1 , what) , (83u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (84u32 ..= 104u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessible_string_property (index - 84u32 + 1 , what) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (106u32 ..= 117u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessible_string_property (index - 106u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (9u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (10u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (11u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (36u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (37u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (38u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (39u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (40u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
                     + {
                         * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (9u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (12u32 ..= 19u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . accessibility_action (index - 12u32 + 1 , action) , (10u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (20u32 ..= 27u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . accessibility_action (index - 20u32 + 1 , action) , (11u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (28u32 ..= 35u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . accessibility_action (index - 28u32 + 1 , action) , (36u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (41u32 ..= 48u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . accessibility_action (index - 41u32 + 1 , action) , (37u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (49u32 ..= 56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . accessibility_action (index - 49u32 + 1 , action) , (38u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (57u32 ..= 64u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . accessibility_action (index - 57u32 + 1 , action) , (39u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (65u32 ..= 72u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . accessibility_action (index - 65u32 + 1 , action) , (40u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (73u32 ..= 80u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . accessibility_action (index - 73u32 + 1 , action) , (83u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (84u32 ..= 104u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . accessibility_action (index - 84u32 + 1 , action) , (5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (106u32 ..= 117u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . accessibility_action (index - 106u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 9u32 => sp :: SupportedAccessibilityAction :: r#Default , 10u32 => sp :: SupportedAccessibilityAction :: r#Default , 11u32 => sp :: SupportedAccessibilityAction :: r#Default , 36u32 => sp :: SupportedAccessibilityAction :: r#Default , 37u32 => sp :: SupportedAccessibilityAction :: r#Default , 38u32 => sp :: SupportedAccessibilityAction :: r#Default , 39u32 => sp :: SupportedAccessibilityAction :: r#Default , 40u32 => sp :: SupportedAccessibilityAction :: r#Default , 9u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 12u32 ..= 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 12u32 + 1) , 10u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 20u32 ..= 27u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 20u32 + 1) , 11u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 28u32 ..= 35u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 28u32 + 1) , 36u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 41u32 ..= 48u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 41u32 + 1) , 37u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 49u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 49u32 + 1) , 38u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 57u32 ..= 64u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 57u32 + 1) , 39u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 65u32 ..= 72u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 65u32 + 1) , 40u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 73u32 ..= 80u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 73u32 + 1) , 83u32 => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 84u32 ..= 104u32 => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 84u32 + 1) , 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 106u32 ..= 117u32 => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 106u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 12u32 ..= 19u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_95 }
                 . apply_pin (_self) . item_element_infos (index - 12u32 + 1) , 20u32 ..= 27u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_96 }
                 . apply_pin (_self) . item_element_infos (index - 20u32 + 1) , 28u32 ..= 35u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_97 }
                 . apply_pin (_self) . item_element_infos (index - 28u32 + 1) , 41u32 ..= 48u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_100 }
                 . apply_pin (_self) . item_element_infos (index - 41u32 + 1) , 49u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_101 }
                 . apply_pin (_self) . item_element_infos (index - 49u32 + 1) , 57u32 ..= 64u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_102 }
                 . apply_pin (_self) . item_element_infos (index - 57u32 + 1) , 65u32 ..= 72u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_103 }
                 . apply_pin (_self) . item_element_infos (index - 65u32 + 1) , 73u32 ..= 80u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_104 }
                 . apply_pin (_self) . item_element_infos (index - 73u32 + 1) , 84u32 ..= 104u32 => {
                     * & Self :: FIELD_OFFSETS . r#editor_area_111 }
                 . apply_pin (_self) . item_element_infos (index - 84u32 + 1) , 106u32 ..= 117u32 => {
                     * & Self :: FIELD_OFFSETS . r#font_window_114 }
                 . apply_pin (_self) . item_element_infos (index - 106u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_tabcomponent_108 {
         r#tabcomponent_108 : InnerTabComponent_root_44 , r#model_data : sp :: Property < sp :: SharedString > , r#model_index : sp :: Property < i32 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_tabcomponent_108 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabDemo > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_tabcomponent_108 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerTabComponent_root_44 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_current_tab }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (31f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_idx }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_name_changed }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_rename_tab) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& (({
                                     * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
                                ) . apply_pin (_self) . get () as _ , args . 0 . clone () as _ ,))) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_select_tab }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab) . apply_pin (x . as_pin_ref ()))) . map (| x | x . set (args . 0 . clone () as _)) ;
                                 }
                             ;
                             {
                                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab_changed) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& (args . 0 . clone () as _ ,))) ;
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_name }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_tab_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((780f64 as f64) / (match & (_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () {
                         x => {
                             x . model_tracker () . track_row_count_changes () ;
                             x . row_count () as i32 }
                         }
                     as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_bar_107_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (12f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerTabComponent_root_44 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + ({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 1u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (31f64 as sp :: Coord , ({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
                 + {
                     * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 12f64 as sp :: Coord ,) , 1u32 ..= 3u32 => return {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#tabcomponent_108 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_tabcomponent_108 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabDemo > ,) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerTabDemo > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             4usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 1u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_tabcomponent_108 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#root_44 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#tabcomponent_108 }
             + {
                 * & InnerTabComponent_root_44 :: FIELD_OFFSETS . r#rectangle_45 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_tabcomponent_108) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_tabcomponent_108 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_tabcomponent_108 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_tabcomponent_108 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_tabcomponent_108 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 82u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_tabcomponent_108 {
         type Data = sp :: SharedString ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_tabcomponent_108 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_tabcomponent_108 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerTabDemo {
         fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = sp :: Rc :: new (SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc))) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             118usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 6u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 81u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 83u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 105u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 106u32 , parent_index : 0u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 1u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 36u32 , parent_index : 1u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 12u32 , parent_index : 6u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 20u32 , parent_index : 6u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 28u32 , parent_index : 6u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 19u32 , parent_index : 9u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 9u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 9u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 12u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 27u32 , parent_index : 10u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 10u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 10u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 28u32 , parent_index : 20u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 35u32 , parent_index : 11u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 9u32 , parent_index : 11u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 10u32 , parent_index : 11u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 11u32 , parent_index : 11u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 12u32 , parent_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 11u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 11u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 36u32 , parent_index : 28u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 41u32 , parent_index : 8u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 49u32 , parent_index : 8u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 57u32 , parent_index : 8u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 65u32 , parent_index : 8u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 7u32 , children_index : 73u32 , parent_index : 8u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 48u32 , parent_index : 36u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 13u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 14u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 15u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 16u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 49u32 , parent_index : 36u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 49u32 , parent_index : 36u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 49u32 , parent_index : 41u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 56u32 , parent_index : 37u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 17u32 , parent_index : 37u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 18u32 , parent_index : 37u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 19u32 , parent_index : 37u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 20u32 , parent_index : 37u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 37u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 37u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 49u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 64u32 , parent_index : 38u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 21u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 22u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 23u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 24u32 , parent_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 65u32 , parent_index : 38u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 65u32 , parent_index : 38u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 65u32 , parent_index : 57u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 72u32 , parent_index : 39u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 25u32 , parent_index : 39u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 26u32 , parent_index : 39u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 27u32 , parent_index : 39u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 28u32 , parent_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 73u32 , parent_index : 39u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 73u32 , parent_index : 39u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 73u32 , parent_index : 65u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 80u32 , parent_index : 40u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 29u32 , parent_index : 40u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 30u32 , parent_index : 40u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 31u32 , parent_index : 40u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 32u32 , parent_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 81u32 , parent_index : 40u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 81u32 , parent_index : 40u32 , item_array_index : 47u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 81u32 , parent_index : 73u32 , item_array_index : 48u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 82u32 , parent_index : 2u32 , item_array_index : 49u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 81u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 84u32 , parent_index : 3u32 , item_array_index : 50u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 88u32 , parent_index : 83u32 , item_array_index : 51u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 89u32 , parent_index : 83u32 , item_array_index : 52u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 89u32 , parent_index : 83u32 , item_array_index : 53u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 105u32 , parent_index : 83u32 , item_array_index : 54u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 89u32 , parent_index : 84u32 , item_array_index : 55u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 90u32 , parent_index : 86u32 , item_array_index : 56u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 93u32 , parent_index : 89u32 , item_array_index : 57u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 95u32 , parent_index : 89u32 , item_array_index : 58u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 100u32 , parent_index : 89u32 , item_array_index : 59u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 94u32 , parent_index : 90u32 , item_array_index : 60u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 95u32 , parent_index : 93u32 , item_array_index : 61u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 96u32 , parent_index : 91u32 , item_array_index : 62u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 99u32 , parent_index : 95u32 , item_array_index : 63u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 99u32 , parent_index : 95u32 , item_array_index : 64u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 100u32 , parent_index : 95u32 , item_array_index : 65u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 100u32 , parent_index : 97u32 , item_array_index : 66u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 101u32 , parent_index : 92u32 , item_array_index : 67u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 104u32 , parent_index : 100u32 , item_array_index : 68u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 104u32 , parent_index : 100u32 , item_array_index : 69u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 105u32 , parent_index : 100u32 , item_array_index : 70u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 105u32 , parent_index : 102u32 , item_array_index : 71u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 106u32 , parent_index : 4u32 , item_array_index : 72u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 107u32 , parent_index : 5u32 , item_array_index : 73u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 108u32 , parent_index : 106u32 , item_array_index : 74u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 109u32 , parent_index : 107u32 , item_array_index : 75u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 111u32 , parent_index : 108u32 , item_array_index : 76u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 112u32 , parent_index : 108u32 , item_array_index : 77u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 112u32 , parent_index : 109u32 , item_array_index : 78u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 113u32 , parent_index : 110u32 , item_array_index : 79u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 114u32 , parent_index : 112u32 , item_array_index : 80u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 115u32 , parent_index : 113u32 , item_array_index : 81u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 117u32 , parent_index : 114u32 , item_array_index : 82u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 117u32 , parent_index : 114u32 , item_array_index : 83u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 118u32 , parent_index : 116u32 , item_array_index : 84u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerTabDemo , sp :: ItemVTable , sp :: AllowPin > ;
             85usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#empty_93 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#rectangle_105 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#editor_area_visibility_110 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_visibility_112 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#root_76 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#empty_94 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#rectangle_98 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#empty_99 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_95 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_96 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_97 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_100 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_101 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_102 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_103 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#_opacity_2 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_touch_area_26 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#i_focus_scope_27 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#button_104 }
             + {
                 * & InnerButton_root_1 :: FIELD_OFFSETS . r#rectangle_3 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#_clip_106 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#root_50 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#_opacity_55 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#background_57 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#contextmenuinternal_58 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#placeholder_75 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#rectangle_56 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#scroll_view_59 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_60 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_visibility_63 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_visibility_69 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#flickable_viewport_61 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#vertical_bar_64 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#border_65 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_opacity_66 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_68 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_67 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#horizontal_bar_70 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#border_71 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_opacity_72 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#touch_area_74 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#thumb_73 }
            ) , sp :: VOffset :: new ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#styled_text_area_113 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#_visibility_77 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 * & InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#rectangle_78 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#root_37 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#_opacity_38 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_opacity_40 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#rectangle_39 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 * & InnerLineEdit_root_37 :: FIELD_OFFSETS . r#background_41 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_28 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#root_clip_33 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#placeholder_34 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#contextmenuinternal_35 }
            ) , sp :: VOffset :: new ({
                 InnerTabDemo :: FIELD_OFFSETS . r#font_window_114 }
             + {
                 InnerFontSizeWindow_root_76 :: FIELD_OFFSETS . r#lineedit_80 }
             + {
                 InnerLineEdit_root_37 :: FIELD_OFFSETS . r#base_43 }
             + {
                 * & InnerLineEditBase_root_28 :: FIELD_OFFSETS . r#text_input_36 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerTabDemo) ;
         }
     ;
     impl sp :: PinnedDrop for InnerTabDemo {
         fn drop (self : core :: pin :: Pin < & mut InnerTabDemo >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerTabDemo {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerTabDemo > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#TabDemo (sp :: VRc < sp :: ItemTreeVTable , InnerTabDemo >) ;
     impl r#TabDemo {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerTabDemo :: new () ? ;
             inner . globals . get () . unwrap () . init () ;
             InnerTabDemo :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_add_new_tab (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_add_new_tab }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_add_new_tab (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_add_new_tab }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_apply_bold (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_bold }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_apply_bold (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_bold }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_apply_font_size (& self , arg_0 : f32 ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_font_size }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_apply_font_size (& self , mut f : impl FnMut (f32) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_font_size }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_apply_underline (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_underline }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_apply_underline (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_apply_underline }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_close_tab (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_close_tab }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_close_tab (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_close_tab }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_current_content (& self) -> sp :: SharedString {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_current_content (& self , value : sp :: SharedString) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 InnerTabDemo :: FIELD_OFFSETS . r#editor_area_111 }
             + {
                 * & InnerTextEdit_root_50 :: FIELD_OFFSETS . r#text_input_62 }
             + sp :: r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_current_tab (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_current_tab (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_current_tab_changed (& self , arg_0 : i32 ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab_changed }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_current_tab_changed (& self , mut f : impl FnMut (i32) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_current_tab_changed }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_font_size (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_font_size (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_font_size }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_is_bold (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_bold }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_is_bold (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_bold }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_is_editing (& self) -> bool {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_is_editing (& self , value : bool) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_is_editing }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_rename_tab (& self , arg_0 : i32 , arg_1 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_rename_tab }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_rename_tab (& self , mut f : impl FnMut (i32 , sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_rename_tab }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         # [allow (dead_code)] pub fn get_tab_contents (& self) -> sp :: ModelRc < sp :: SharedString > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_contents }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_tab_contents (& self , value : sp :: ModelRc < sp :: SharedString >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_contents }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_tab_titles (& self) -> sp :: ModelRc < sp :: SharedString > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_tab_titles (& self , value : sp :: ModelRc < sp :: SharedString >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_tab_titles }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_update_tab_content (& self , arg_0 : sp :: SharedString ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_update_tab_content }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_update_tab_content (& self , mut f : impl FnMut (sp :: SharedString) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerTabDemo :: FIELD_OFFSETS . r#root_91_update_tab_content }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] fn get_tool_tab_width (& self , _private_property : ()) {
             }
         # [allow (dead_code)] fn set_tool_tab_width (& self , _private_property : ()) {
             }
         }
     impl From < r#TabDemo > for sp :: VRc < sp :: ItemTreeVTable , InnerTabDemo > {
         fn from (value : r#TabDemo) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#TabDemo {
         type Inner = InnerTabDemo ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerTabDemo >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_CupertinoPalette_127 : :: core :: pin :: Pin < sp :: Rc < InnerCupertinoPalette_127 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> Self {
             Self {
                 global_CupertinoPalette_127 : InnerCupertinoPalette_127 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
             }
         fn init (self : & sp :: Rc < Self >) {
             self . global_CupertinoPalette_127 . clone () . init (self) ;
             }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg xmlns=\"http://www.w3.org/2000/svg\" height=\"24px\" viewBox=\"0 -960 960 960\" width=\"24px\" fill=\"#e8eaed\">\n  <path d=\"m321-80-71-71 329-329-329-329 71-71 400 400L321-80Z\"/>\n</svg>\n" ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_10_0 = slint :: VersionCheck_1_10_0 ;
     }
 # [allow (unused_imports)] pub use slint_generatedTabDemo :: {
     r#TabDemo , r#TextStyle , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
