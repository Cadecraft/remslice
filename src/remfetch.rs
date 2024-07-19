use crate::remdata;
use crate::utils;

/// Fetch the info
pub fn remfetch(current_remdata: &remdata::RemData) -> String {
    let mut res: String = String::new();
    // TODO: impl all details
    // Like neofetch/fastfetch, but for remslice
    // TODO: impl ASCII logo (a morning star? an orange slice?)
    let current_morningstar = if current_remdata.get_m() { "[success]" } else { "[failure]" };
    res.push_str(&format!("                          ,.           REMSLICE\n"));
    res.push_str(&format!("                        == =-\\         --------\n"));
    res.push_str(&format!("                      ==.  O  \\        REM/Recent Version: {}\n", current_remdata.get_r()));
    res.push_str(&format!("                    ==   O_____|       REM/Edit Date:      {}\n", current_remdata.get_e()));
    res.push_str(&format!("                  ==  ____   O |       REM/Morning Star:   {}\n", current_morningstar));
    res.push_str(&format!("                ==____        =        Current Time:       {}\n", utils::get_time_formatted()));
    res.push_str(&format!("              ==]_  O  . .   /         OS:                 {}\n", utils::get_os()));
    res.push_str(&format!("            == | ] .        =          \n"));
    res.push_str(&format!("          ==  |    ]  O   O/           \n"));
    res.push_str(&format!("        ==    | O    ]    =            \n"));
    res.push_str(&format!("      ==  .  |    .   ]==-             \n"));
    res.push_str(&format!("    /=      |  .O   ==-                \n"));
    res.push_str(&format!("     -===  |  =====-                   \n"));
    res.push_str(&format!("         -===-                         \n"));
    res.push_str(&format!("                                       \n"));
    /*res.push_str(&format!("                                       REMSLICE"));
    res.push_str(&format!("                                       --------"));
    res.push_str(&format!("                                       REM: Recent Version: {}", current_remdata.get_r()));
    res.push_str(&format!("                                       REM: Edit Date:      {}", current_remdata.get_e()));
    res.push_str(&format!("                                       REM: Morning Star:   {}", current_morningstar));
    res.push_str(&format!("                                       Current Time:        {}", utils::get_time_formatted()));
    res.push_str(&format!("                                       OS: {}", utils::get_os()));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));
    res.push_str(&format!("                                       "));*/
    res
}
