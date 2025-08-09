// src/lingua/ga/gramadoir.rs
use libperl_rs::*;
use libperl_sys::*;
use std::cell::RefCell;
use std::convert::TryInto;
use std::ffi::CString;

// ---- helpers from the libperl-rs examples ----
use super::super::super::eg::sv0::*;

// One interpreter per thread, initialized once with a caching bridge
thread_local! {
    static PERL: RefCell<Option<Perl>> = RefCell::new(None);
}

fn ensure_perl_inited() -> Result<(), String> {
    PERL.with(|cell| {
        if cell.borrow().is_some() {
            return Ok(());
        }

        let mut perl = Perl::new();

        let r1 = perl.parse(&["", "-MGramBridge", "-e0"], &[]);
        
        if r1 != 0 {
            return Err("Failed to load GramBridge (perl.parse returned non-zero)".into());
        }

        eprintln!("Perl interpreter initialized with GramBridge loaded.");

        // Check that method exists: GramBridge->can('check')
        let can_check = call_list_method(
            &mut perl,
            "GramBridge".into(),
            "can".into(),
            vec!["grammatical_errors".to_string()],
        )?;

        if can_check.is_empty() {
            return Err("GramBridge->grammatical_errors not found (method missing)".into());
        } else {
            println!("Habemus GramBridge->grammatical_errors method");
        }

        *cell.borrow_mut() = Some(perl);
        Ok(())
    })
}

/// Call GramBridge->check(text, lang) and get Vec<String> of XML snippets
pub fn grammatical_errors(text: &str) -> Result<Vec<String>, String> {
    let _ = ensure_perl_inited()?;
    PERL.with(|cell| {
        let mut binding = cell.borrow_mut();
        let perl: &mut Perl = binding
            .as_mut()
            .expect("Perl not initialised (should be impossible)");

        let can_check = call_list_method(
            perl,
            "GramBridge".into(),
            "can".into(),
            vec!["grammatical_errors".to_string()],
        )?;

        // Call a Perl *method* with a class invocant ("GramBridge")
        let svs = call_list_method(perl, "GramBridge".into(), "grammatical_errors".into(),
                                   vec![text.to_owned()])?;

        // Convert SVs to Strings (using your sv0 helpers)
        svs.into_iter()
            .map(|sv| match sv {
                Sv::SCALAR { pv: Some(pv), .. } => Ok(pv.clone()),
                Sv::SCALAR { ivuv, .. } => Ok(format!("{:?}", ivuv)),
                Sv::REF(inner) => match *inner {
                    Sv::SCALAR { pv: Some(ref pv), .. } => Ok(pv.clone()),
                    _ => Err("unexpected REF value in SV".to_string()),
                },
                _ => Err("unexpected non-scalar SV in return list".to_string()),
            })
            .collect()
    })
}

fn call_list_method(perl: &mut Perl, class: String, method: String, args: Vec<String>) -> Result<Vec<Sv>, String> {
    let my = perl.my_perl();

    unsafe_perl_api! { Perl_push_scope(my) }
    unsafe_perl_api! { Perl_savetmps(my) }

    // dSP
    let mut sp = unsafe { (*my).Istack_sp };

    // PUSHMARK(SP)
    perl.pushmark(sp);

    // XPUSHs(invocant + args)
    sp = unsafe_perl_api! { Perl_stack_grow(my, sp, sp, (1 + args.len()).try_into().unwrap()) };
    sp_push!(sp, perl.str2svpv_mortal(class.as_str()));
    for a in &args { sp_push!(sp, perl.str2svpv_mortal(a)); }

    // PUTBACK
    unsafe { (*my).Istack_sp = sp; }

    // call
    let c_method = CString::new(method).map_err(|_| "null in method".to_string())?;
    let count = unsafe_perl_api! { Perl_call_method(my, c_method.as_ptr(), (G_METHOD_NAMED|G_LIST) as i32) };

    // SPAGAIN
    let sp_after = unsafe { (*my).Istack_sp };

    // collect from (SP - count + 1 ..= SP)
    let mut res = Vec::with_capacity(count as usize);
    let mut src = unsafe { sp_after.sub(count as usize - 1) };
    for _ in 0..count {
        let sv = unsafe { *src };
        res.push(sv_extract(sv));
        src = unsafe { src.add(1) };
    }

    // pop return values: SP -= count
    unsafe { (*my).Istack_sp = sp_after.sub(count as usize); }

    perl.free_tmps();
    unsafe_perl_api! { Perl_pop_scope(my) }
    Ok(res)
}

fn stack_extract(perl: &Perl, count: perl_stack_size_t) -> Vec<Sv> {
    let mut res = Vec::new();
    let mut src = unsafe { (*(perl.my_perl)).Istack_base.add(1) };
    for _ in 0..count {
        let sv = unsafe { *src };
        res.push(sv_extract(sv)); // from your sv0
        src = unsafe { src.add(1) };
    }
    res
}
