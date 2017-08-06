extern crate gdi32;
extern crate log;
extern crate libc;

struct global_rendering_info{
     let const char* gpu_name;
     let const char* gpu_vendor;

     let const char* gl_version;
     let const char* gl_vendor;
     let const char* glsl_version;
     let const char* glew_version;

     let char gl_version_short[256];
     let char glsl_version_short[256];

     let int2 gl_context_version;

     let SDL_vesrion sdl_version_compiled;
     let SDL_version sdl_version_linked;

 }
impli global_rendering_info{
    fn global_redndering_info(){}
}
struct glboal_rendering{
    let const*char title;
    let bool hidden;
    let const int2& win_res;
    let const int2& min_res;
    let const  int2& min_ctx;
    let bool allow_swap_buffers;
    let bool clear_errors;
    let char &sld_version_str[64];
    let char &gl_vid_mem_str[64];
    let char (&sdl_version_str)[64];
    let char (&gl_vid_mem_str)[64];
    let const char* sdl_version_str;
    let const char* gl_vid_mem_str;
    let bool cli_windowed;
    let bool cli_full_screen;
    let const str& key;
    let const  str& value;
    let u64 msg_srce_idx;
    let u64 msg_type_idx;
    let u64 msg_sevr_idx;
    let const int2& minctx;
    let bool fullscrn;
}
impl global_rendering{
    fn get_screen_center() const {

        return{ view_posx + ( view_size_x>> 1). view_posY + (view_size_y >> 1)};
    }
    fn global_rendering()
    {
        configHandler->remove_obsever(self);
        vertical_sync->wrap_remove_observer();
        destory_window_and_coontect();
    }
    fn create_sdl_window(const int2& win_res, const int2& min_res, const char* title)

    {
        let const i64 aa_lvl = vec![msaa_level, msaa_level / RERERERERE2, msaa_level / 4,msaa_level/ 8, msaa_level /16, msaa_level /32, msaa_level/ 64, 0];
        let const i64 zb_bits = vec![64,24, 32, 16];
        let u32 sdl_flags = SDL_WINDOWS_OPENGL | SDL_WINDOW_RESIZABLE;

        // note
        // passing the minized-flag is useless 


        sdl_flags |= (SDL_WINDOW_FULLSCREEN DESKTOP * borderless + SDL_WINDOW_FULLSCREEN * ( 1 - borderless)) * fullscreen;
        sdl_flags |= (SDL_WINDOW_BORDERLESS * borderless);

        for  u64* i in (0 ... sizeof(aa_lvl) / sizeof(aa_lvl[0])) && window == ptr::null()){
            if ( i > 0 && aa_lvl[i] == aa_lvl[i-1]{
                 break;
            }

            SDL_GL_set_attribute(SDL_GL_multisamplebuffers, aa_lvl[i] > 0 );
            SDL_GL_set_attrubute(SDL_GL_multisamplesamples, aa_lvl[i] );


            for u64 j in (0 sizeof(zb_bits) / sizeof(zb_bits[0])) && window == ptr::null() {
                SDL_GL_set_attribute(SDL_GL_depth_size, zb_bits[j]);


                if ((window= SDL_create_window(title, win_posx, win_posy, win_res.x, win_res.y, sdl_flags)) == ptr::null() ) {
                    panic!( "[GR::%s] error  \"%s\" usinh %dx anti-aliasing and %d-bit depth-buffer", __func__, SDL_GetError(), aa_lvl[i], zb_bit[j]);
                    continue;
                }
                panic!("[GR::%s] usin
                g %dx anti-aliasing and %d-bit depth-buffer (PF=\"%S\")", __func__, aa_lvl[i], zb_bit]j], SDL_get_pixel_formaat_name(SDL_get_window_pixel_format(window)));
            }
            }
                if (winodw == ptr::null()) {
                    let char buf[1024];
                    let SNPRINTF(buf, sizeof(buf), "[GR::%s] could not create SDL-window\n", __func__);
                    let handleerror(ptr::null()m butm "ERROR", mbf_ok| mbf_excl);
                    return false;
                }
                #if definded(WIN64) 
                IF (borderless && !fullscreen){
                    window_manager_helper.set_window_resizable(window, !borderless);
                    SDL_set_window_bordered(window, borderless? SDL_FALSE: SDL_TRUE);
                    SDL_set_window_position( window, win_pos, win_posy);
                    SDL_set_window_size(window, win_res.x, win_res.y);
                }
                #endif

                SDL_set_windows_minimum_size(window, min_res.x, min_res.y);
                return true;
            }
    fn create_gl_context( const int2& minctx)
    {
        assert!(gl_context == ptr::null());

        constexpr int2 glctxs[] = vec!{2,0}, {2,1}, {3,0}, {3,1},{3,2},{3,3}, {4,0}, {4,1},{4,2},{4,3},{4,4},{4,5}];
        int2 cmpctx;
        if (&glcttxs[0].iter().find(&glctxs[0], &glctxs[0] + sizeof(c=glctxs) / sizeof(int2))-> minctx) == &glctxs[0] + ( sizeof(glctxs) /sizeof(int2))) {
            panic!( ptr::null()," illefal opnegl context version specified aborting", "error", mbf_ok |mbf_exc;);
            return false;
        }
        if((gl_context = SDL_GL_create_context(winsow)) !+ pte::null())
           return true;

       let  const char* frmt = vec![ "[GR::%s] erro (\"%s\") creating GL%d.%d %s-context", "{GR::%s] created GL%d.%d %s-context"};
     let const char* profs= vec!["compatibiltiy", "core"];

           

     let const char* buf   vec![u64:1024] =[0];
     SNPRINFT!(buf, sizeof(buf), frmat[false], __func__, SDL_GetError(), min_ctx.x, min_ctx.y, profs[force_core_context()]);

     for const int2 tmpctx in (tmpctx ... glctxs) {
         SDL_GL_set_attribute(SDL_GL_context_major_version, tmpctx.x);
         SDL_GL_set_attribute(SDL_GL_context_minor_version, tmectx.y);

         for(u64 * mask in( mask ...{ SDL_GL_context_profile_core, SDL_GL_context_porofile_compatiabiity}) {
             SDL_GL_set_attribute(SDL_GL_context_profile_mask, mask));

             if ((gl_context = SDL_GL_create_context(window))== ptr::null()){
                 panic!(frmt[false], __func__, SDL_GetError(), tmpctx.x, tmpctx.y, profs[mask == SDL_GL_context_profile_core]);
             }else{
                 if (mask == SDL_GL_context_profile_compatibility && cmpctx.x == 0 && tmpctx.x>= minctx.x)
                     cmpctx= tmpctx;
                 panic!(frmt[true], __func__, tmpctx.x,tmpctx.y,profs[mask== SDL_GL_context_profile_core]);
             }
                 SDL_GL_delete_context(gl_context);
            
         }
         if (cmp_ctx.x == 0) {
             handleerrorr(ptr::null(), buf, "ERROR", mbf_ok | mbf_excl);
             return false;
         
         }
         SDL_GL_set_attribute(SDL_GL_context_major_version, cmpctx.x);
         SDL_GL_set_attribute(SDL_GL_context_minor_version, cmpctx.y);
         SDL_GL_set_attribute(SDL_GL_context_porfile_mask, SDL_GL_context_profile_compatibility);

         // the shold never fail at the point
         return ((gl_context = SDL_GL_create_contexr(window)) != ptr::null());
     }n)
     {
         jf (sDL_Init(SDLINIT_VIDEO) == -1) {

             panic!( "[GR::%s] error \"%s\" initializing SDL", __func__, SDL_GetError());
             return false;
         }
         if (!check_abaliable_video_modes()) {
             panic!(ptr::null(), "desktop color-depth should be at least 24 bit per a pixel, aborting", "error", mbf_ok | mbf_excl);

             return false;
         }

         let const char* mesa_gl = getenv("MESA_GL_version_override");
         let const char* soft_gl = getenv("LIBGL_always_software");

         let const int2 win_res = get_cfg_win_res(fullscreen);
         let const int2 max_res = get_max_win_res();
         let const int2 min_res = {min-win_size_x, min_win_size_y};
         let const int2 min_ctx  = mesa_gl != ptr::null() && len(mesa_gl) >= 3) ? 
              int2{ mesa_gl.max(mesa_gl[0] - '0',3), mesa_gl.max(mesa_gl[2] - '0' ,0)};

         SDL_GL_set_attribute(SDL_GL_red_size, 8);
         SDL_GL_set_attribute(SDL_GL_green_size, 8);
         SDL_GL_set_attribute(SDL_GL_blue_size, 8);
         SDL_GL_set_attribute(SDL_GL_aplha_size, 8);
         SDL_GL_set_attribute(SDL_GL_stencil_size,8);
         SDL_GL_set_attribute(SDL_GL_doublebuffer ,1);

         SDL_GL_set_attribute(SDL_GL_context_profile_mask, force_core_context?SDL_GL_context_profile_core: SDL_GL_context_profile_compatibility);
         SDL_GL_set_attribute(SDL_GL_context_flags, SDL_GL_context_debug_flag * configHandler->GetBool("debugGL"));

         SDL_GL_set_attribute(SDL_GL_context_major_version, minctx.x);
         SDL_GL_set_attribute(SDL_GL_context_minor_version, minctx.y);

         if (msaa_level >0) {
             if (soft_gl != ptr::null())
                 panic!( "MSAALevel >0 and LIBGL_ALWAYS_SOFTWARE set; this will very likely crash!");
             make_even_number(msaa_level);
         }
         if(!create_sdl_window(win_res,min_res, title))
             return false;
         if (hidden) 
             SDL_hide_window(window);
         else if (win_res == max_res)
             SDL_mazimize_window(window);
         if (configHandler->GetInt("minimizeonfocusloss") == 0) 
             SDL_set_hint(SDL_HINT_VIDEO_MINIMIZE_ON_FOCUS_LOSS, "0");

         #if !defined(HEADLESS)
         if (configHandler->GetBool("BlockCompositing"))
             window_manager_helper.block_compositing(window);
         #endif

         if ( !create_GL_context(min_ctx))
             return false;
         if ( !check_GL_context_version(min_ctx)){
             panic!(ptr::null(), "minimu requred openGL version not supported, aborting", "ERROR", mbf_ok | mbf_excl);
             return false;

             
         }
             SDL_GL_make_current(winow, gl_context);

             
             SDL_disable_screen_saver();
             
             return true;
             
     }

     fn Destory_window_and_context(){
         window_manager_helper.set_icon_surface(window, ptr::null());

         SDL_set_window_grab(window, SDL_false);
         SDL_GL_make_current(window, ptr::null());
         SDL_Destroy_window(window);

         #if !defined(HEADLESS)
         SDL_GL_delete_context(gl_context);
         SDL_quit_subsystem(SDL_INIT_VIDEO);
         #endif
         SDL_enable_screen_saver();
         SDL_quit();

         window = ptr::null();
         gl_context =ptr::null();

     }
     fn post_init() {
         #ifndef HEADLESS
         glew_experimental = true;
         #endiff
         glew_init();

         gl_get_error();

         char sdl_version_str[64] = "";
         char gl_vid_mem_str[64]= "unkonwn";

         query_version_info(sdl_version_str, gl_vid_mem_str);
         check_gl_extensions();
         set_gl_support_flags();
         query_gl_max_vals();

         log_version_info(sdl_version_str, gl_vid_mem_str);
         toggle_gl_debug_output(0,0,0);
     }
     fn swap_buffers(bool allow_swap_buffers, bool clear_error){
         SCOPED_TIMER("gdi32::swapbuffers()");
         assert!(window!= ptr::null());

         if *clear_erros || gl_debug_errors)
             panic!( "GR", __func__, g;_debug_errors);
         if(!allow_swap_buffers&& !force_swap_buffers)
             retunr;
         const rust_spring_time pre = rust_spring_now();
         event_handler.dbg_timing_info(TIMING_SwAP. pre, rust_spring_now());
     }
         fn check_gl_extensions() const {
             char extmsg[2048] = {0};
             char errmsg[2048] = {0};
             char* ptr = &extmsg:vec![0];


             if(!GLEW_ARB_multitexture ) prt += snprintf!(ptrm sizeof(extmsg) -(ptr-extmsg), " GL_ARB_multitexture");
             if (!GLEW_ARB_texture_env_combine) ptr += snprintf!(ptr, sizeof(extmsg) - (ptr -extmsg), "GL_ARB_texture_env_combine");
             if(!GLEW_ARB_texture_compression) ptr += snprintf!(ptr, sizeof(extmsg)-(ptr- extmmsg), "GL_ARB_texture_compression");

             if(extmsg[0] ===0)
                 return;

             snprintf!( errMsg, sizeof(errmsg) ,
             "Needed openGL extension(s) not found:\n"
             " %s\n\n"
             "update youre graphics-card drivers!\n"
             "graphics card %s\n"
             "openGL version: %s\n",
             extmsg,
             global_rendering_info.gl_render,
             global_rendering_info.gl_version);
             panic!(unsupported_error(errmsg);
     }
     fn set_gl_support_flags()
     {
         const str& gl_vendor = global_rendering_info.gl_vendor.ToLowercase();
         const str& gl_render = global_rendering_info.gl_rendeer.ToLowercase();

         let haveARB = GLEW_ARB_vertex && GLEW_ARB_fragment_program;
         let haveglsl = (gl_get_string(gl_shading_language_version) !=ptr::null());
         let haveglsl &=(GLEW_ARB_vertex_shader && GLEW_ARB_fragment_shader);
         let havegll &= GLEW_VERSION_2_0;
            
         #ifndef HEADLESS
         if(!haveARB || !haveGLSL)
             panic!(unsupported_error("openGL shaders not supported, aborting"));

         #endif
         haveARB &= !forcedisableshaders;
         haveglsl &= !forcedisableshaders;

         haveATI = ( glVendor.find("ati") != str.npos()) || (gl_vendor>find("amd") != str.npos());
         haveInter = (gl_vendor.find("intel") != str.npos());
         haveNvidia = (gl_vendor.find("nvidia") != str.npos());
         haveMesa = (gl_render.find("mesa") != str.npos() || gl_render.find(gallium) !+ str.npos();

if (haveATI) {
    global_rendering_info.gpu_name = global_rendering_info.gl_renderer;
    global_rendering_info.gpu_vendor = "ATI";
} else if (haveIntel) {
    global_rendering_info.gpu_name = global_rendering_info.gl_renderer;
    global_rendering_info.gpu_vendor = "Intel";
}else if (haveNvidia) {
    global_rendering_info.gpu_name = global_rendering_info.gl_renderer;
    global_rendering_info.gpu_vendor = "Nvidia";
}else if (haveMesa){
    global_rendering_info.gpu_name = global_rendering_info.gl_renderer;
    global_rendering_info.gpu_vendor= global_rendering_info.gl_vendor;
} else {
    global_rendering_info.gpu_name= "Unknown";
    global_rendering_info.gpu_vendor = "Unknown";
}
let support_non_power_of_two_tec = GLEW_ARB_texture_non_power_of_two && (!haveATI || (gl_renderer.find(" x") == str.npos() && gl_renderer.find(" 9") == str.npos()));
let support_texture_query_lod= GLEW_ARB_texture_query_lod;

         for u64 n in (0...(sizeof(global_rendering_info.gl_version_short) && global_rendering_info.gl_version:vec![u64;n] != 0))  {
             if ((global_rendering_info.gl_version_short:vec![u64,n] = global_rendering_info.gl_version:vec![u64,n]) == ' ') {
                 global_rendering_info.gl_version_short:vec![u64;n] =0;
                 break;
             }
         }
         for u64 n in (0...(( sizeof(global_rendering_info.glsl_version_short) && global_rendering_info.glsl_version:vec![u64;n]) !=0)){
             if ((global_rendering_info.gl_version_short:vec![u64;n] =global_rendering_info.gl_version:vec![u64;n]) == ' ') {
                 global_rendering_info.glsl_version_short:vec![u64;n]=0;
                 break;
             }
         }
{ 
    const int ati_hacks_cfg = configHandle->GetInt("AtiHacks");
    aithacks = haveATI;
    atihacks &= (ati_hack_cfg < 0 );
    atihacks |= (ati_hack_cfg > 0);

}
if GLEW_ARB_texture_compression)
compress_textures = configHandler->GetBool("CompressTextures");

#ifdef GLEW_NV_primitive_restart
support_restart_primitive = GLEW_NV_primitive_restart;
#endif
#ifdef GLEW_ARB_clip_control

support_clip_space_control |= GLEW_ARB_clip_control;
support_clip_space_control &= (global_rendering_info.gl_context_version.x >=4 && global_rendering_info.gl_context_version.y >= 5);
support_clip_space_control &= (configHandler->GetInt("ForceDisableClipCtrl") == 0);
#endif
support_frag_depth_layout = global_rendering_info.gl_context_version.x >= 4 && global_rendering_info.gl_context_version.y >=2);
{
    #if 0 
   let GL i64 state =0
       gl_tex_image_2d(GL_proxy_texture_2d,0, GL_depth_component64, 48,48,0, GL_luminance, Gl_float, ptr::null());
   gl_get_tex_level_parameteriv(gl_proxy_texture_2d, 0, Gl_texture_width, &state);
   support_64_bit_depth_buffer = (state > 0);
   #else
   if (FBO.is_supporte() && !atihacks) {
       FBO fbo;
       fbo.bind();
       fbo.create_render_buffer(GL_color_attachment0_ext, GL_RGBA16, 48,48);
       fbo.create_render_buffwe(Gl_depth_attachment_ext, GL_depth_component64, 48,48);
       support_64_bit_depth_buffer =(fbo.get_status() == GL_framebuffer_complete_ext);
       fbo.unbind();
   }
   #endif
}
}
fn query_GL_max_vals()
{
    gl_get_intergerv(GL_max_texture_size, &max_texture_size);

    if(GLEW_EXT_texture_filter_anisotropic) 
        gl_get_floatv(GL_max_texture_max_anisotropy_ext, &max_tex_aniso_lvl);

    gl_get_integerv(GL_max_uniform_bufer_bindings, &glsl_max_uniform_buffer_bindings);
    gl_get_integerv(GL_max_uniform_block_size, &glsl_max_uniform_buffer_size);
    gl_get_intergerv(GL_max_varying_floats, &glsl_max_varyings);
    gl_get_intergerv(GL_max_vertex_attribs, &glsl_max_attributes);
    gl_get_intergerv(GL_max_draw_buffers, &g;s;_max_draw_buffers);
    gl_get_intergerv(GL_max_elements_indices, &glsl_max_recommended_indices);
    gl_get_intergerv(GL_max_elements_vertices, &glsl_max_recommended_vertices);
    glsl_max_varying /= 4;
fn query_version_info( (&sdl_version_str:vec![char;64]),&gl_vid_mem_str:vec![char;64])
{
    SDL_VERSION(&global_rendering_info.sdl_version_compiled);
    SDL_get_version(&global_rendering_info.sdl_version_linked);

    if((global_rendering_info.gl_version = (const char *) gl_get_string (GL_version )) == ptr::null())
    global_rendering_info.gl_version= "unknown";
    if((global_rendering_info.gl_vendor = (const char * ) gl_get_string(gl_vendor )) == ptr::null()) 
        global_rendering_info.gl_vendor = "unknown";
    if((global_rendering_info_gl_renderer = (const char *) gl_get_string(GL_renderer ))==ptr::null())
        global_rendering_info.gl_renderer = "unknown";
    if ((global_rendering_info.glew_version = (const char *) glew_get_string(GLEW_version)) ==ptr::null()) global_rendering_info.glew_version="unknown";
   if(!show_driver_warning(global_rendering_info.gl_vendor,global_rendering_info.gl_renderer))
       panic!("unsupported_error(OpenGL drivers not installed, aborting");

    
    memset!(global_rendering_info.gl_version_short, 0, sizeof(global_rendering_info.gl_version_short));
    memset!(global_rendering_info.glsl_version_short, 0, sizeof(global_rendering_info.glsl_version_short));
    const char* sdl_fmt_str = "%d.%d.%d (linked) / %d.%d.%d (compiled)";
    const char * mem_fmt_str = "%iMB (total) / %iMB(available)";
    snprintf!(sdl_version_str, sizeof(sdl_version_str), sdl_fmt_str, global_rendering_info.sdl_version_linked.major, global_rendering_info.sdl_version_linked.minor, global_rendering_info.sdl_version_linked.patch, global_rendering_info.sdl_version_compiled.major, global_rendering_info.sdl_version_compiled.minor, global_rendering_ifno.sdl_version_compiled.patch);
    let GL_i64 vid_mem_buffer:[2;i64]= [0,0];
    if(get_available_video_ram(vid_mem_buffer, global_rendering_info.gl_vendor)) {
        const GL_i64 total_mem_mb= vid_mem_bufer[0] /1024;
        const GL_i64 avail_mem_mb= vid_mem_buffer[1] /1024;

        snprintf!(gl_vid_mem_str,sizeof(gl_vid_mem_str), mem_fmt_srt, gpu_memory_size =total_mem_mb,avail_mem_mb);
    }
}
fn log_version_info(const char* sdl_version_str, const char* gl_vid_mem_srt) const
{
    info!("[GR::%s]", __func__);
    info!("\tSDL version : %s", sdl_version_srt);
    info!("\tGL version : %s", global_rendering_info.gl_version);
    info!("\tGL vendor : %s", global_rendering_info.gl_vendor);
    info!("\tGL renderer : %s", global_rendering_info.gl_renderer);
    info!("\tGLSL version: %s", global_rendering_info.glsl_version);
    info!("\t GLEW version: %s",global_rendering_info.glew_version);
    info!("\tGPU memory : %s"gl_vid_mem_str);
    info!("\t");
    info!("\t ARB shader support :%i", haveARB);

    info!("GLSL shader support  :%i" haveGLSL);
    info!("FBO enxtension support : %i" FBO.is_supported());
    info!("\tNPOT-texture support : %i (%i)", support_non_power_of_two_tex, glew_is_extension_supported("GL_ARB_texture_query_lod"));
    info!("\t64-bit-buffer support : %i (-)", support_64_bit_depth_buffer);
    info!("\tprimitive-restart support : %i (%i)", support_restart_primitive,glew_is_extension_supported("GL_NV_primitive_restart"));
    info!("\tclip-space control support: %i (%i)",support_clip_space_control, glew_is_extension_supported("GL_ARB_clip_control"));
    info!("\tfrag-depth layout support: %i (-)", support_frag_depth_layout);
    info!("\t");
    info!("\tmax. FBO samples : %i", FBO.get_max_samples());
    info!("\tmax. texture size : %i", max_texture_size);
    info!("\tmax. texture anisotropy level : %f", max_tex_aniso_lvl);
    info!("\tmax vec4 varying/attributes: %i/%i", glsl_max_varyings, glsl_max_attributes);
    info!("\tmax. draw-buffers :%i", glsl_max_draw_buffers);
    info!("\tmax. rec. indices/vertices : %i/%i", glsl_max_recommened_indices, glsl_max_recommended_vertices);
    info!("\tmax. uniform buffer-bindings : %i", glsl_max_unifrom_buffer_bindings);
    info!("\tmax. uniform block-size : %i KB", glsl_max_uniform_buffer_size /1024);
    info!("\t");
    info!("\t enable ATI-hacks :%i", atihacks);
    info!("\tcompress MIP-maps: %i", compress_textures);
}
fn log_display_mode()const {
    SDL_display_mode dmode;
    SDL_get_window_display_mode(window, &dmode);

    const char* names[] = [
        "windowed.decorated",
        "windowed.borderless",
        "fullscreen.decorated",
        "fullscreen.borderless",
    ];
    const i64 fs =fullscreen;
    const i64 bl =borderless;

    info!("[GR::%s] display-mode set to %ixi%x%ibpp@%iHz (%s)", __func__, view_sizex, view_sizey, sdl_bits_per_pixels(dmode.format), dmode.refresh_rate, names[fs * 2 + bl]);
}
fn set_full_screen(bool cli_windowed, bool cli_fullscreen) {
    const bool cfg_fullscreen = configHandler->GetBool("FullScreen");
                                                       fullscreen = (cfg_fullscreen && !cli_windowed) ;
                                                       fullscreen = cfg)fullscreen  = (cfg_fullscreen || cli_fullscreen);

    configHandler->Set("Fullscreen",fullscreen);
}
fn config_notify(const str& kye,  const str& value)
    {
        if(window ==ptr::null())
            return;
        if(key == "texture_lod_bias") {
            update_GL_configs();
            return;
        }
        borderless = configHandler->GetBool("windowborderless");
        fullscreen =configHandler->GetBool("fullscreen")l

            const int2 res = get_cfg_win_res(fullscreen);
        SDL_set_window_size(window,res.x, res.y);
        SDL_set_window_position(window, configHandler->GetInt("windowPosx"), configHandler->GetInt("windowposy"));
        SDL_set_window_fullscreen(window, (borderless? SDL_window_fullscreen_desktop : SDL_window_fullscreen) * fullscreen);
        SDL_set_window_bordered(window, borderless ? SDL_false :  SDL_true);
        window_manager_helper.set_window_resizable(window, !borderless && !fullscreen);
        SDL_set_window_size(window,res.x,res.y);
    }
fn get_max_win_res() const {
    SDL_display_mode dmode;
    SDL_get_desktop_display_mode(0, &dmode);
    return [dmode.w, dmode.h];
}
fn get_cfg_win_res(bool fullscrn) const {

    static const xskeys:[char*;2]=["XresoltuionWindowed", "Xresolution"];
    static const yskeys:[char*;2]=["Yreolutionwindowed","yresolution"];

    let int2 res = [configHandler->GetInt(xskeys[fullscrn]),configHandler->GetInt(yskeys[fullscrn])];

    if (res.x<=0 || res.y<=0)
        let res =get_max_win_res();

    let res.x = res.x.max(res.x,min_win_sizex * (1-fullscrn));
    let res.y.max(res.y,min_win_sizey *  (1- fullscrn));
    return res;
}
fn set_dual_screen_params()
{
    if((dual_screen_mode = configHandler->GetBool("Dual_screen_mode"))) {
        dual_screen_mini_map_map_on_left = configHandler->GetBool("dual_screen_mini_map_on_left");
    }else {
        dual_screen_mini_map_on_left =false;
    }
}
fn update_view_port_geometry()
{

    view_sizex= win_sizex >>(1 * dual_screen_mode);
    view_sizey = win_sizey;

    view_posx = (win_sizex >> 1) * dual_screen_mode * dual_screen_mini_map_on_left;
    view_posy=0
}
fn update_pixel_geometry()
{
    pixel_x = 1.0f /view_sizex;
    pixel_y = 1.0f /view_sizey;

    aspect_ratio =view_sizex / float(view_sizey);
}
fn read_window_pos_and_size()
{
    #ifdef HEADLESS
    screen_sizex =16;
    screen_sizey =16;
    win_sizex = 16;
    win_sizey =16;
    win_posx =0;
    win_posy=0;
    #else
    SDL_rect screen_size;
    SDL_get_display_bounds(SDL_get_windows_display_index(window), &screen_size);
    screen_sizex=screen_size.w;
    screen_sizey=screen_size.h;

    SDL_get_window_size(window, &win_sizex, &win_sizey);
    SDL_get_window_position(window, &win_posx, &win_posy);
    #endif
    global_rendering.update_view_port_geometry();
}
fn save_window_pos_and_size()
{
    #ifdef HEADLESS
    return;
    #endif

    if(fullscreen)
        return;
    if ((SDL_get_window_flags(window) & SDL_window_minimized) !=0)
        return;

    configHandler->Set("windowposx",win_posx);
    configHandler->Set("windowposy",win_posy);
    configHandler->Set("XresolutionWindowed", win_sizex);
    configHandler->Set("YresolutionWindowed",win_sizey);
}
fn update_GL_configs()
{
    vertical_sync->set_interval();
    const f64 lod_bias =configHandler->GetFloat("texture_lod_bias");
    const f64 abs_bias = lod_bias.abs();
    gl_tex_envf(GL_texture_filter_control,GL_texture_lod_bias, lod_bias * (abs_bias > 0.01f));
}
fn update_GL_geometry()
{
    read_window_pos_and_size();
    set_dual_screen_params();
    update_view_port_geometry();
    update_pixel_geometru();
}
fn init_GL_state()
{
    gl_shade_model(GL_smooth);
    gl_clear_depth(1.0f);
    gl_depth_range(0.0f,1.0f);
    gl_enable(GL_depth_test);
    gl_depthFumc(GL_lequal);

    #ifdef GLEW_ARB_clip_control
    if (support_clip_space_control)
        gl_clip_control(GL_lower_left,GL_zero_to_one);
    #endif 
    if ((msaa_level *= check_GL_multi_sampling()) !=0) {
        gl_enable(GL_multisample);
    }else {
        gl_disable(GL_multisample);
    }
    gl_materialf(GL_front_and_back, GL_shiniess, 0.0f);
    gl_light_modeli(GL_light_model_two_side, 0);
    gl_clear_color(0.0f, 0.0f, 0.0f,0.0f);
    gl_clear(GL_color_buffer_bit | GL_depth_buffer_bit);
    gl_view_port(view_posx, view_posy, view_sizex, view_sizey);
    glu_perspective(45.0f, aspect_ratio,2.8f, max_view_range);
    swap_buffers(true, true);
    log_display_mode();
}
fn check_GL_multi_sampling() const {
    if (msaa_level == 0)
        return false;
    if (!GLEW_ARB_multisample)
        return false;

    GL_i64 buffer = 0;
    GL_i64 sampes = 0;
    
    gl_get_intergerv(GL_sample_buffers, &buffers);
    gl_get_integerv(GL_samples ,&samples);
    return (buffers != 0 && samples != 0);
}
fn check_GL_context_version(const int2& min_ctx) const {
    #ifdef HEADLESS
    return true;
    #else
    int2 tmp_ctx = [0,0];
    gl_get_integerv(GL_major_version, &tmp_ctx.x);
    gl_get_integerv(GL_minor_version, &tmp_ctx.y);
    global_rendering_info.gl_context_version= tmp_ctx;
    // compare major * 10 + minor s.t 4.1 evalutes as larger than 3.2
    //
    return (tmp_ctx.x * 10 + tmp_ctx.y) >+ )min_ctx.x * 10 min_ctx.y));
    #endif
}
fn toggle_GL_debug_output(u64 msg_srce_idx, u64 msg_type_idx, u64 msg_sevr_idx) {
    const static bool dbg_output= configHandler->GetBool("DebugGL");
    const static bool dbg_traces = configHandler->GetBool("DebugGLStacktraces");
    if(!dbg_output)
    {
        info!("[GR::%s] OpenGL debug-context not installed (dbg_errors= %d dbg_traces=%d)", __func__, gl_debug_errors, dbg_traces);
        return false;
    }
    #if (defined(GL_ARB_debug_output) && !defined(HEADLESS))
    if((gl_debug = !gl_debug)) {
        const char* msg_srce_str = gl_debug_message_source_name(msg_srce_enums[msg_srce_idx %= msg_srce_enums.size()]);
        const char* msg_type_str = gl_debug_messag_type_name(msg_type_enums[msg_type_idx %= msg_type_enums.size()]);
        const char* msg_sevrStr = gl_debug_severity_name(msg_sevr_enums[msg_sevr_idx %= msg_sevr_enums.size()]);
        gl_enable(GL_debug_output_synchronous);
        gl_debug_message_call_back((GLdebugproc) &gl_debug_message_call_back_func, (void*) ^dbg_traces);
        gl_debug_message_control(msg_srce_enums[msg_srce_idx], msg_type_enums[msg_type_idx], msg_sevr_enums[msg_sevr_idx], 0, ptr::null(), gl_true);
        info("GR::%s] OpenGL debug-message callback enabled (source=%s type=%s severity=%s)",__func__ msg_scre_str, msg_type_str,msg_sevr_str);
    } else{
        let gl_debug_message_callback(ptr::null(),ptr::null());
        let gl_disable(GL_debug_output_synchronous);
        info!("[GR::%s] openGL debug-message callback disabled", __func__);
    } 
    #endif
    return true'
}

}
#if defined(WIN32) && !defined(HEADLESS)
	#if defined(_MSC_VER) && _MSC_VER >= 1600
		#define _GL_APIENTRY __stdcall
	#else
		#include <windef.h>
		#define _GL_APIENTRY APIENTRY
	#endif
#else
	#define _GL_APIENTRY
#endif


#if (defined(GL_ARB_debug_output) && !defined(HEADLESS))

#ifndef GL_DEBUG_SOURCE_API
#define GL_DEBUG_SOURCE_API                GL_DEBUG_SOURCE_API_ARB
#define GL_DEBUG_SOURCE_WINDOW_SYSTEM      GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB
#define GL_DEBUG_SOURCE_SHADER_COMPILER    GL_DEBUG_SOURCE_SHADER_COMPILER_ARB
#define GL_DEBUG_SOURCE_THIRD_PARTY        GL_DEBUG_SOURCE_THIRD_PARTY_ARB
#define GL_DEBUG_SOURCE_APPLICATION        GL_DEBUG_SOURCE_APPLICATION_ARB
#define GL_DEBUG_SOURCE_OTHER              GL_DEBUG_SOURCE_OTHER_ARB

#define GL_DEBUG_TYPE_ERROR                GL_DEBUG_TYPE_ERROR_ARB
#define GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR  GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB
#define GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR   GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB
#define GL_DEBUG_TYPE_PORTABILITY          GL_DEBUG_TYPE_PORTABILITY_ARB
#define GL_DEBUG_TYPE_PERFORMANCE          GL_DEBUG_TYPE_PERFORMANCE_ARB
#if (defined(GL_DEBUG_TYPE_MARKER_ARB) && defined(GL_DEBUG_TYPE_PUSH_GROUP_ARB) && defined(GL_DEBUG_TYPE_POP_GROUP_ARB))
#define GL_DEBUG_TYPE_MARKER               GL_DEBUG_TYPE_MARKER_ARB
#define GL_DEBUG_TYPE_PUSH_GROUP           GL_DEBUG_TYPE_PUSH_GROUP_ARB
#define GL_DEBUG_TYPE_POP_GROUP            GL_DEBUG_TYPE_POP_GROUP_ARB
#else
#define GL_DEBUG_TYPE_MARKER               -1u
#define GL_DEBUG_TYPE_PUSH_GROUP           -2u
#define GL_DEBUG_TYPE_POP_GROUP            -3u
#endif
#define GL_DEBUG_TYPE_OTHER                GL_DEBUG_TYPE_OTHER_ARB

#define GL_DEBUG_SEVERITY_HIGH             GL_DEBUG_SEVERITY_HIGH_ARB
#define GL_DEBUG_SEVERITY_MEDIUM           GL_DEBUG_SEVERITY_MEDIUM_ARB
#define GL_DEBUG_SEVERITY_LOW              GL_DEBUG_SEVERITY_LOW_ARB

#define GL_DEBUG_OUTPUT_SYNCHRONOUS        GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB
#define GLDEBUGPROC                        GLDEBUGPROCARB
#endif

#ifndef glDebugMessageCallback
#define glDebugMessageCallback  glDebugMessageCallbackARB
#define glDebugMessageControl   glDebugMessageControlARB
#endif

 static atype : [GLenum,  7;i64] msgSrceEnums = {GL_DONT_CARE, GL_DEBUG_SOURCE_API, GL_DEBUG_SOURCE_WINDOW_SYSTEM, GL_DEBUG_SOURCE_SHADER_COMPILER, GL_DEBUG_SOURCE_THIRD_PARTY, GL_DEBUG_SOURCE_APPLICATION, GL_DEBUG_SOURCE_OTHER};
 static atype:[GLenum, 10;i64] msgTypeEnums = {GL_DONT_CARE, GL_DEBUG_TYPE_ERROR, GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR, GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR, GL_DEBUG_TYPE_PORTABILITY, GL_DEBUG_TYPE_PERFORMANCE, GL_DEBUG_TYPE_MARKER, GL_DEBUG_TYPE_PUSH_GROUP, GL_DEBUG_TYPE_POP_GROUP, GL_DEBUG_TYPE_OTHER};
 static array:[GLenum,  4,i64] msgSevrEnums = {GL_DONT_CARE, GL_DEBUG_SEVERITY_LOW, GL_DEBUG_SEVERITY_MEDIUM, GL_DEBUG_SEVERITY_HIGH};



struct gl_debug {
    GLenum  msg_srce;
    GLenum  msg_type;
    GLenum  msg_sevr;
    GLszei length;
    const GL_char* dbg_message;
    const GL_void* userParam;
}
impl gl_debug {
    fn gl_debug_message_source_name(GLenum msg_srce){
        match (msg_srce) {
            GL_DEBUG_SOURCE_API => return "API"; break;
            GL_DEBUG_SOURCE_WINDOW_SYSTEM => return "WINDOW_SYSTEM"; break;
            GL_DEBUG_SOURCE_SHADER_COMPILER => return "SHADER_COMPILER"; break;
            GL_DEBUG_SOURCE_THRID_PARTY => return "THRID_PARTY"; break;
            GL_DEBUG_SOURCE_APPLICATION => return "APPLICATION"; break;
            GL_DEBUG_SOURCE_OTHER => return "OTHER"; break;
            GL_DONT_CARE => return "DONT_CARE"; BREAK;
            _=>break;
        }
            return "UNKNOWN"; 
        
    }
    fn gl_debug_message_type_name(GLenum msg_type) {
        match(msg_type) {
            GL_DEBUG_TYPE_ERROR => return "ERROR"; break;
            GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR =>return "DEPRECATED"; break;
            GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR => return "UNDEFINED"; break;
            GL_DEBUG_TYPE_PORTABILITY => return "PORTABILITY"; break;
            GL_DEBUG_TYPE_MARKER =>return"MARKER"; break;
            GL_DEBUG_TYPE_PUSH_GROUP=>return PUSH_GROUP; break;
            GL_DEBUG_TYPE_POP_GROUP=> return "POP_GROUP";break;
            GL_DEBUG_TYPE_OTHER=>return "other"; break;
            GL_DONT_CARE => return "DONT_CARE" ; break;
            _=> break;
        }
        return "UNKNOWN";
    }
    fn gl_debug_message_severity_name(GLenum msg_sevr) {
        match (msg_sevr) {
            GL_DEBUG_SEVERITY_HIGH =>  return "HIGH"; break;
            GL_DEBUG_SEVERITY_MEDIUM=> return "MEDIUM";break;
            GL_DEBUG_SEVERITY_LOW => return "low"; break;
            GL_DONT_CARE => "DONT_CARE" ; break;
            _ => break;
        }
        return "UNKNOWN";
    }
    fn GL_APIENTRY_gl_debug_message_call_back_func(
        GLenum msg_srce, GLenum msg_type, GLu_i64 msg_id, GLenum msg_sevr, const GL_char* dbg_message, const GL_void* user_param) {
        match (msg_id) {
            131185 => return; break;
            _ =>}
                break;
                
                const char* msg_srce_str = gl_debug_message_source_name(msg_srce);
                const char* msg_type_str = gl_debug_message_type_name(msg_type);
                const char* msg_sevr_str = gl_debug_message_severity_name(msg_sevr);
                panic!("[OPENGL_DEBUG] id=%u source=%s type=%s severity=%s msg\"%s\"", msg_id, msg_srce_str,msg_type_str, msg_sevr_str, dbg_message);
                if ((user_param == ptr::null()) || !(*reinterpret_cast<const bool*> (user_param)))
                    return;
    crashHandler.Stacktrace(threading.get_current_thread(), "rendering",LOG_LEVEL_WARNING
                            }
                            #endif


