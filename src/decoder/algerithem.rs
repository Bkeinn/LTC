mod daten;
use daten::*;
use ndarray::Array3;

pub fn get_variable(charactar_base: char, depth: u8, for_char: char, dataset: &Array3<f64>) -> f64 {
    return *(dataset
        .get((
            converter(charactar_base),
            converter(for_char),
            depth as usize,
        ))
        .unwrap());
    // match charactar_base {
    //     'a' => return get_value_from_for_char(for_char, get_exact_array_A(depth)),
    //     'b' => return get_value_from_for_char(for_char, get_exact_array_B(depth)),
    //     'c' => return get_value_from_for_char(for_char, get_exact_array_C(depth)),
    //     'd' => return get_value_from_for_char(for_char, get_exact_array_D(depth)),
    //     'e' => return get_value_from_for_char(for_char, get_exact_array_E(depth)),
    //     'f' => return get_value_from_for_char(for_char, get_exact_array_F(depth)),
    //     'g' => return get_value_from_for_char(for_char, get_exact_array_G(depth)),
    //     'h' => return get_value_from_for_char(for_char, get_exact_array_H(depth)),
    //     'i' => return get_value_from_for_char(for_char, get_exact_array_I(depth)),
    //     'j' => return get_value_from_for_char(for_char, get_exact_array_J(depth)),
    //     'k' => return get_value_from_for_char(for_char, get_exact_array_K(depth)),
    //     'l' => return get_value_from_for_char(for_char, get_exact_array_L(depth)),
    //     'm' => return get_value_from_for_char(for_char, get_exact_array_M(depth)),
    //     'n' => return get_value_from_for_char(for_char, get_exact_array_N(depth)),
    //     'o' => return get_value_from_for_char(for_char, get_exact_array_O(depth)),
    //     'p' => return get_value_from_for_char(for_char, get_exact_array_P(depth)),
    //     'q' => return get_value_from_for_char(for_char, get_exact_array_Q(depth)),
    //     'r' => return get_value_from_for_char(for_char, get_exact_array_R(depth)),
    //     's' => return get_value_from_for_char(for_char, get_exact_array_S(depth)),
    //     't' => return get_value_from_for_char(for_char, get_exact_array_T(depth)),
    //     'u' => return get_value_from_for_char(for_char, get_exact_array_U(depth)),
    //     'v' => return get_value_from_for_char(for_char, get_exact_array_V(depth)),
    //     'w' => return get_value_from_for_char(for_char, get_exact_array_W(depth)),
    //     'x' => return get_value_from_for_char(for_char, get_exact_array_X(depth)),
    //     'y' => return get_value_from_for_char(for_char, get_exact_array_Y(depth)),
    //     'z' => return get_value_from_for_char(for_char, get_exact_array_Z(depth)),
    //     // ' ' => return get_value_from_for_char(for_char, get_exact_array_SPACE(depth)),
    //     _ => panic!("get varibale did not get an supported char ->{charactar_base}<-"),
    // }
}

const fn converter(characer: char) -> usize {
    return match characer {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        ' ' => 26,
        '.' => 27,
        _ => panic!("This character is not supported"),
    };
}

fn get_exact_array_A(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return A_nach_3,
        -2 => return A_nach_2,
        -1 => return A_nach_1,
        1 => return A_befor_1,
        2 => return A_befor_2,
        3 => return A_befor_3,
        4 => return A_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_B(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return B_nach_3,
        -2 => return B_nach_2,
        -1 => return B_nach_1,
        1 => return B_befor_1,
        2 => return B_befor_2,
        3 => return B_befor_3,
        4 => return B_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_C(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return C_nach_3,
        -2 => return C_nach_2,
        -1 => return C_nach_1,
        1 => return C_befor_1,
        2 => return C_befor_2,
        3 => return C_befor_3,
        4 => return C_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_D(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return D_nach_3,
        -2 => return D_nach_2,
        -1 => return D_nach_1,
        1 => return D_befor_1,
        2 => return D_befor_2,
        3 => return D_befor_3,
        4 => return D_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_E(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return E_nach_3,
        -2 => return E_nach_2,
        -1 => return E_nach_1,
        1 => return E_befor_1,
        2 => return E_befor_2,
        3 => return E_befor_3,
        4 => return E_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_F(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return F_nach_3,
        -2 => return F_nach_2,
        -1 => return F_nach_1,
        1 => return F_befor_1,
        2 => return F_befor_2,
        3 => return F_befor_3,
        4 => return F_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_G(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return G_nach_3,
        -2 => return G_nach_2,
        -1 => return G_nach_1,
        1 => return G_befor_1,
        2 => return G_befor_2,
        3 => return G_befor_3,
        4 => return G_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_H(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return H_nach_3,
        -2 => return H_nach_2,
        -1 => return H_nach_1,
        1 => return H_befor_1,
        2 => return H_befor_2,
        3 => return H_befor_3,
        4 => return H_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_I(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return I_nach_3,
        -2 => return I_nach_2,
        -1 => return I_nach_1,
        1 => return I_befor_1,
        2 => return I_befor_2,
        3 => return I_befor_3,
        4 => return I_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_J(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return J_nach_3,
        -2 => return J_nach_2,
        -1 => return J_nach_1,
        1 => return J_befor_1,
        2 => return J_befor_2,
        3 => return J_befor_3,
        4 => return J_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_K(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return K_nach_3,
        -2 => return K_nach_2,
        -1 => return K_nach_1,
        1 => return K_befor_1,
        2 => return K_befor_2,
        3 => return K_befor_3,
        4 => return K_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_L(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return L_nach_3,
        -2 => return L_nach_2,
        -1 => return L_nach_1,
        1 => return L_befor_1,
        2 => return L_befor_2,
        3 => return L_befor_3,
        4 => return L_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_M(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return M_nach_3,
        -2 => return M_nach_2,
        -1 => return M_nach_1,
        1 => return M_befor_1,
        2 => return M_befor_2,
        3 => return M_befor_3,
        4 => return M_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_N(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return N_nach_3,
        -2 => return N_nach_2,
        -1 => return N_nach_1,
        1 => return N_befor_1,
        2 => return N_befor_2,
        3 => return N_befor_3,
        4 => return N_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_O(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return O_nach_3,
        -2 => return O_nach_2,
        -1 => return O_nach_1,
        1 => return O_befor_1,
        2 => return O_befor_2,
        3 => return O_befor_3,
        4 => return O_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_P(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return P_nach_3,
        -2 => return P_nach_2,
        -1 => return P_nach_1,
        1 => return P_befor_1,
        2 => return P_befor_2,
        3 => return P_befor_3,
        4 => return P_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_Q(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return Q_nach_3,
        -2 => return Q_nach_2,
        -1 => return Q_nach_1,
        1 => return Q_befor_1,
        2 => return Q_befor_2,
        3 => return Q_befor_3,
        4 => return Q_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_R(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return R_nach_3,
        -2 => return R_nach_2,
        -1 => return R_nach_1,
        1 => return R_befor_1,
        2 => return R_befor_2,
        3 => return R_befor_3,
        4 => return R_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_S(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return S_nach_3,
        -2 => return S_nach_2,
        -1 => return S_nach_1,
        1 => return S_befor_1,
        2 => return S_befor_2,
        3 => return S_befor_3,
        4 => return S_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_T(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return T_nach_3,
        -2 => return T_nach_2,
        -1 => return T_nach_1,
        1 => return T_befor_1,
        2 => return T_befor_2,
        3 => return T_befor_3,
        4 => return T_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_U(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return U_nach_3,
        -2 => return U_nach_2,
        -1 => return U_nach_1,
        1 => return U_befor_1,
        2 => return U_befor_2,
        3 => return U_befor_3,
        4 => return U_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_V(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return V_nach_3,
        -2 => return V_nach_2,
        -1 => return V_nach_1,
        1 => return V_befor_1,
        2 => return V_befor_2,
        3 => return V_befor_3,
        4 => return V_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_W(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return W_nach_3,
        -2 => return W_nach_2,
        -1 => return W_nach_1,
        1 => return W_befor_1,
        2 => return W_befor_2,
        3 => return W_befor_3,
        4 => return W_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_X(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return X_nach_3,
        -2 => return X_nach_2,
        -1 => return X_nach_1,
        1 => return X_befor_1,
        2 => return X_befor_2,
        3 => return X_befor_3,
        4 => return X_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_Y(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return Y_nach_3,
        -2 => return Y_nach_2,
        -1 => return Y_nach_1,
        1 => return Y_befor_1,
        2 => return Y_befor_2,
        3 => return Y_befor_3,
        4 => return Y_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}
fn get_exact_array_Z(depth: i16) -> [f32; 28] {
    match depth {
        -3 => return Z_nach_3,
        -2 => return Z_nach_2,
        -1 => return Z_nach_1,
        1 => return Z_befor_1,
        2 => return Z_befor_2,
        3 => return Z_befor_3,
        4 => return Z_befor_4,
        _ => panic!("get_exact_array varibale did not get an supported i16"),
    }
}

// fn get_exact_array_SPACE(depth: i16) -> [f32; 28] {
//     match depth {
//         -3 => return SPACE_nach_3,
//         -2 => return SPACE_nach_2,
//         -1 => return SPACE_nach_1,
//         1 => return SPACE_befor_1,
//         2 => return SPACE_befor_2,
//         3 => return SPACE_befor_3,
//         4 => return SPACE_befor_4,
//         _ => panic!("get_exact_array varibale did not get an supported i16"),
//     }
// }
// You give this funktion one of arrays and for which character posibility you are searching and it gives you the f32
fn get_value_from_for_char(for_char: char, character_array: [f32; 28]) -> f32 {
    // println!("for_char :{}:", for_char);
    match for_char {
        'a' => character_array[0],
        'b' => character_array[0 + 1],
        'c' => character_array[0 + 1 + 1],
        'd' => character_array[0 + 1 + 1 + 1],
        'e' => character_array[0 + 1 + 1 + 1 + 1],
        'f' => character_array[0 + 1 + 1 + 1 + 1 + 1],
        'g' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1],
        'h' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'i' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'j' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'k' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'l' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'm' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'n' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'o' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'p' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'q' => character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1],
        'r' => {
            character_array[0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1]
        }
        's' => {
            character_array
                [0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1]
        }
        't' => {
            character_array
                [0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1]
        }
        'u' => {
            character_array
                [0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1]
        }
        'v' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        'w' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        'x' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        'y' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        'z' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        '.' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        ' ' => {
            character_array[0
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1
                + 1]
        }
        _ => panic!("Did not get a good for_char ->{for_char}<-"),
    }
}
