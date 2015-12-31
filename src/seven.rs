use std::collections::HashMap;

pub static INPUT: &'static str = "NOT dq -> dr\nkg OR kf -> kh\nep OR eo -> eq\n44430 -> b\nNOT gs -> gt\ndd OR do -> dp\neg AND ei -> ej\ny AND ae -> ag\njx AND jz -> ka\nlf RSHIFT 2 -> lg\nz AND aa -> ac\ndy AND ej -> el\nbj OR bi -> bk\nkk RSHIFT 3 -> km\nNOT cn -> co\ngn AND gp -> gq\ncq AND cs -> ct\neo LSHIFT 15 -> es\nlg OR lm -> ln\ndy OR ej -> ek\nNOT di -> dj\n1 AND fi -> fj\nkf LSHIFT 15 -> kj\nNOT jy -> jz\nNOT ft -> fu\nfs AND fu -> fv\nNOT hr -> hs\nck OR cl -> cm\njp RSHIFT 5 -> js\niv OR jb -> jc\nis OR it -> iu\nld OR le -> lf\nNOT fc -> fd\nNOT dm -> dn\nbn OR by -> bz\naj AND al -> am\ncd LSHIFT 15 -> ch\njp AND ka -> kc\nci OR ct -> cu\ngv AND gx -> gy\nde AND dk -> dm\nx RSHIFT 5 -> aa\net RSHIFT 2 -> eu\nx RSHIFT 1 -> aq\nia OR ig -> ih\nbk LSHIFT 1 -> ce\ny OR ae -> af\nNOT ca -> cb\ne AND f -> h\nia AND ig -> ii\nck AND cl -> cn\nNOT jh -> ji\nz OR aa -> ab\n1 AND en -> eo\nib AND ic -> ie\nNOT eh -> ei\niy AND ja -> jb\nNOT bb -> bc\nha OR gz -> hb\n1 AND cx -> cy\nNOT ax -> ay\nev OR ew -> ex\nbn RSHIFT 2 -> bo\ner OR es -> et\neu OR fa -> fb\njp OR ka -> kb\nea AND eb -> ed\nk AND m -> n\net RSHIFT 3 -> ev\net RSHIFT 5 -> ew\nhz RSHIFT 1 -> is\nki OR kj -> kk\nNOT h -> i\nlv LSHIFT 15 -> lz\nas RSHIFT 1 -> bl\nhu LSHIFT 15 -> hy\niw AND ix -> iz\nlf RSHIFT 1 -> ly\nfp OR fv -> fw\n1 AND am -> an\nap LSHIFT 1 -> bj\nu LSHIFT 1 -> ao\nb RSHIFT 5 -> f\njq AND jw -> jy\niu RSHIFT 3 -> iw\nih AND ij -> ik\nNOT iz -> ja\nde OR dk -> dl\niu OR jf -> jg\nas AND bd -> bf\nb RSHIFT 3 -> e\njq OR jw -> jx\niv AND jb -> jd\ncg OR ch -> ci\niu AND jf -> jh\nlx -> a\n1 AND cc -> cd\nly OR lz -> ma\nNOT el -> em\n1 AND bh -> bi\nfb AND fd -> fe\nlf OR lq -> lr\nbn RSHIFT 3 -> bp\nbn AND by -> ca\naf AND ah -> ai\ncf LSHIFT 1 -> cz\ndw OR dx -> dy\ngj AND gu -> gw\njg AND ji -> jj\njr OR js -> jt\nbl OR bm -> bn\ngj RSHIFT 2 -> gk\ncj OR cp -> cq\ngj OR gu -> gv\nb OR n -> o\no AND q -> r\nbi LSHIFT 15 -> bm\ndy RSHIFT 1 -> er\ncu AND cw -> cx\niw OR ix -> iy\nhc OR hd -> he\n0 -> c\ndb OR dc -> dd\nkk RSHIFT 2 -> kl\neq LSHIFT 1 -> fk\ndz OR ef -> eg\nNOT ed -> ee\nlw OR lv -> lx\nfw AND fy -> fz\ndz AND ef -> eh\njp RSHIFT 3 -> jr\nlg AND lm -> lo\nci RSHIFT 2 -> cj\nbe AND bg -> bh\nlc LSHIFT 1 -> lw\nhm AND ho -> hp\njr AND js -> ju\n1 AND io -> ip\ncm AND co -> cp\nib OR ic -> id\nNOT bf -> bg\nfo RSHIFT 5 -> fr\nip LSHIFT 15 -> it\njt AND jv -> jw\njc AND je -> jf\ndu OR dt -> dv\nNOT fx -> fy\naw AND ay -> az\nge LSHIFT 15 -> gi\nNOT ak -> al\nfm OR fn -> fo\nff AND fh -> fi\nci RSHIFT 5 -> cl\ncz OR cy -> da\nNOT ey -> ez\nNOT ju -> jv\nNOT ls -> lt\nkk AND kv -> kx\nNOT ii -> ij\nkl AND kr -> kt\njk LSHIFT 15 -> jo\ne OR f -> g\nNOT bs -> bt\nhi AND hk -> hl\nhz OR ik -> il\nek AND em -> en\nao OR an -> ap\ndv LSHIFT 1 -> ep\nan LSHIFT 15 -> ar\nfo RSHIFT 1 -> gh\nNOT im -> in\nkk RSHIFT 1 -> ld\nhw LSHIFT 1 -> iq\nec AND ee -> ef\nhb LSHIFT 1 -> hv\nkb AND kd -> ke\nx AND ai -> ak\ndd AND do -> dq\naq OR ar -> as\niq OR ip -> ir\ndl AND dn -> do\niu RSHIFT 5 -> ix\nas OR bd -> be\nNOT go -> gp\nfk OR fj -> fl\njm LSHIFT 1 -> kg\nNOT cv -> cw\ndp AND dr -> ds\ndt LSHIFT 15 -> dx\net RSHIFT 1 -> fm\ndy RSHIFT 3 -> ea\nfp AND fv -> fx\nNOT p -> q\ndd RSHIFT 2 -> de\neu AND fa -> fc\nba AND bc -> bd\ndh AND dj -> dk\nlr AND lt -> lu\nhe RSHIFT 1 -> hx\nex AND ez -> fa\ndf OR dg -> dh\nfj LSHIFT 15 -> fn\nNOT kx -> ky\ngk OR gq -> gr\ndy RSHIFT 2 -> dz\ngh OR gi -> gj\nlj AND ll -> lm\nx OR ai -> aj\nbz AND cb -> cc\n1 AND lu -> lv\nas RSHIFT 3 -> au\nce OR cd -> cf\nil AND in -> io\ndd RSHIFT 1 -> dw\nNOT lo -> lp\nc LSHIFT 1 -> t\ndd RSHIFT 3 -> df\ndd RSHIFT 5 -> dg\nlh AND li -> lk\nlf RSHIFT 5 -> li\ndy RSHIFT 5 -> eb\nNOT kt -> ku\nat OR az -> ba\nx RSHIFT 3 -> z\nNOT lk -> ll\nlb OR la -> lc\n1 AND r -> s\nlh OR li -> lj\nln AND lp -> lq\nkk RSHIFT 5 -> kn\nea OR eb -> ec\nci AND ct -> cv\nb RSHIFT 2 -> d\njp RSHIFT 1 -> ki\nNOT cr -> cs\nNOT jd -> je\njp RSHIFT 2 -> jq\njn OR jo -> jp\nlf RSHIFT 3 -> lh\n1 AND ds -> dt\nlf AND lq -> ls\nla LSHIFT 15 -> le\nNOT fg -> fh\nat AND az -> bb\nau AND av -> ax\nkw AND ky -> kz\nv OR w -> x\nkk OR kv -> kw\nks AND ku -> kv\nkh LSHIFT 1 -> lb\n1 AND kz -> la\nNOT kc -> kd\nx RSHIFT 2 -> y\net OR fe -> ff\net AND fe -> fg\nNOT ac -> ad\njl OR jk -> jm\n1 AND jj -> jk\nbn RSHIFT 1 -> cg\nNOT kp -> kq\nci RSHIFT 3 -> ck\nev AND ew -> ey\n1 AND ke -> kf\ncj AND cp -> cr\nir LSHIFT 1 -> jl\nNOT gw -> gx\nas RSHIFT 2 -> at\niu RSHIFT 1 -> jn\ncy LSHIFT 15 -> dc\nhg OR hh -> hi\nci RSHIFT 1 -> db\nau OR av -> aw\nkm AND kn -> kp\ngj RSHIFT 1 -> hc\niu RSHIFT 2 -> iv\nab AND ad -> ae\nda LSHIFT 1 -> du\nNOT bw -> bx\nkm OR kn -> ko\nko AND kq -> kr\nbv AND bx -> by\nkl OR kr -> ks\n1 AND ht -> hu\ndf AND dg -> di\nNOT ag -> ah\nd OR j -> k\nd AND j -> l\nb AND n -> p\ngf OR ge -> gg\ngg LSHIFT 1 -> ha\nbn RSHIFT 5 -> bq\nbo OR bu -> bv\n1 AND gy -> gz\ns LSHIFT 15 -> w\nNOT ie -> if\nas RSHIFT 5 -> av\nbo AND bu -> bw\nhz AND ik -> im\nbp AND bq -> bs\nb RSHIFT 1 -> v\nNOT l -> m\nbp OR bq -> br\ng AND i -> j\nbr AND bt -> bu\nt OR s -> u\nhz RSHIFT 5 -> ic\ngk AND gq -> gs\nfl LSHIFT 1 -> gf\nhe RSHIFT 3 -> hg\ngz LSHIFT 15 -> hd\nhf OR hl -> hm\n1 AND gd -> ge\nfo OR fz -> ga\nid AND if -> ig\nfo AND fz -> gb\ngr AND gt -> gu\nhe OR hp -> hq\nfq AND fr -> ft\nga AND gc -> gd\nfo RSHIFT 2 -> fp\ngl OR gm -> gn\nhg AND hh -> hj\nNOT hn -> ho\ngl AND gm -> go\nhe RSHIFT 5 -> hh\nNOT gb -> gc\nhq AND hs -> ht\nhz RSHIFT 3 -> ib\nhz RSHIFT 2 -> ia\nfq OR fr -> fs\nhx OR hy -> hz\nhe AND hp -> hr\ngj RSHIFT 5 -> gm\nhf AND hl -> hn\nhv OR hu -> hw\nNOT hj -> hk\ngj RSHIFT 3 -> gl\nfo RSHIFT 3 -> fq\nhe RSHIFT 2 -> hf";

enum Wire {
    Wait,
    Value(i32)
}

enum Gate {
    Input(i32),
    Link(String),
    And(String, String),
    Or(String, String),
    Not(String),
    Lshift(String, i32),
    Rshift(String, i32)
}

type Board = HashMap<String, Wire>;

impl Gate {
    fn eval(&self, board: &Board) -> Wire {
        use self::Gate::*;
        use self::Wire::*;

        match *self {
            Input(val)  => Value(val),
            Link(ref a)     => *
            board.get(a).unwrap_or(&Wire::Wait),
            And(a, b)   => if let (Some(Value(a)), Some(Value(b))) = (board.get(a), board.get(b)) { Value(a & b) } else { Wait },
            Or(a, b)    => if let (Some(Value(a)), Some(Value(b))) = (board.get(a), board.get(b)) { Value(a | b) } else { Wait },
            Not(a)      => if let Some(Value(a)) = board.get(a) { Value(!a) } else { Wait },
            _           => Wait
        }
    }
}

pub fn solve(input: &str) -> i32 {
    0
}

// use std::collections::HashMap;
//
// pub fn solve(input: &str) -> i32 {
//     let mut program: Vec<Instr> = input.split("\n").map(|s| From::from(s)).collect();
//     let mut wires: HashMap<&String, i32> = HashMap::new();
//     loop {
//         let count = run(&program, &wires);
//         if count == 0 { break; }
//     }
//     *wires.read1(&"c".into()).unwrap()
// }
//
// fn run<'a>(program: &'a Vec<Instr>, wires: &HashMap<&'a String, i32>) -> i32 {
//     let mut executed = 0;
//     for instr in program {
//         if instr.execute(&mut wires) { executed += 1; }
//     }
//     executed
// }
//
// trait Circuit {
//     fn read1(&self, s: &String) -> Option<&i32>;
//     fn read2(&self, a: &String, b: &String) -> Option<(&i32, &i32)>;
// }
//
// impl<'a> Circuit for HashMap<&'a String, i32> {
//     fn read1(&self, s: &String) -> Option<&i32> {
//         self.get(s)
//     }
//
//     fn read2(&self, a: &String, b: &String) -> Option<(&i32, &i32)> {
//         match (self.get(a), self.get(b)) {
//             (Some(a), Some(b)) => Some((a, b)),
//             _ => None
//         }
//     }
// }
//
// #[derive(Debug)]
// enum Signal {
//     Wait,
//     Value(i32)
// }
//
// #[derive(Debug)]
// enum Op {
//     Input(i32),
//     Read(String),
//     And(String, String),
//     Or(String, String),
//     Not(String),
//     Lshift(String, i32),
//     Rshift(String, i32)
// }
//
// impl Op {
//     fn eval(&self, wires: &HashMap<&String, i32>) -> Signal {
//         match *self {
//             Op::Input(val)          => Some(Signal::Value(val)),
//             Op::Read(ref a)         => wires.read1(a)   .map(|a|      Signal::Value(*a)),
//             Op::And(ref a, ref b)   => wires.read2(a, b).map(|(a, b)| Signal::Value(a & b)),
//             Op::Or(ref a, ref b)    => wires.read2(a, b).map(|(a, b)| Signal::Value(a | b)),
//             Op::Not(ref a)          => wires.read1(a)   .map(|a|      Signal::Value(!a)),
//             Op::Lshift(ref a, amt)  => wires.read1(a)   .map(|a|      Signal::Value(a << amt)),
//             Op::Rshift(ref a, amt)  => wires.read1(a)   .map(|a|      Signal::Value(a >> amt)),
//         }.unwrap_or(Signal::Wait)
//     }
// }
//
// #[derive(Debug)]
// struct Instr(Op, String);
//
// impl Instr {
//     fn execute<'a>(&'a self, wires: &mut HashMap<&'a String, i32>) -> bool {
//         if let Signal::Value(value) = self.0.eval(&wires) {
//             wires.insert(&self.1, value);
//             true
//         } else {
//             false
//         }
//     }
// }
//
// impl<'a> From<&'a str> for Instr {
//     fn from(input: &'a str) -> Self {
//         let parts = input.split(" -> ").collect::<Vec<&str>>();
//         let lhs   = parts.get(0).unwrap_or_else(|| panic!("LHS missing"));
//         let rhs   = parts.get(1).unwrap_or_else(|| panic!("RHS missing"));
//         Instr(parse_op(lhs), (*rhs).into())
//     }
// }
//
// fn parse_op(input: &str) -> Op {
//     let parts = input.split(" ").collect::<Vec<_>>();
//     match parts.len() {
//         1 => parse_op_1(parts),
//         2 => parse_op_2(parts),
//         3 => parse_op_3(parts),
//         _ => panic!("Illegal operation")
//     }
// }
//
// fn parse_op_1(parts: Vec<&str>) -> Op {
//     let value = parts.get(0).unwrap();
//     match value.parse::<i32>() {
//         Ok(value) => Op::Input(value),
//         Err(_)    => Op::Read((*value).into())
//     }
// }
//
// fn parse_op_2(parts: Vec<&str>) -> Op {
//     let op  = parts.get(0).unwrap();
//     let rhs = parts.get(1).unwrap();
//     match *op {
//         "NOT" => Op::Not((*rhs).into()),
//         _     => panic!("Unknown operation")
//     }
// }
//
// fn parse_op_3(parts: Vec<&str>) -> Op {
//     let lhs = parts.get(0).unwrap();
//     let op  = parts.get(1).unwrap();
//     let rhs = parts.get(2).unwrap();
//     match *op {
//         "AND"    => Op::And((*lhs).into(), (*rhs).into()),
//         "OR"     => Op::Or((*lhs).into(), (*rhs).into()),
//         "LSHIFT" => Op::Lshift((*lhs).into(), (*rhs).parse::<i32>().unwrap_or_else(|_| panic!("Value expected"))),
//         "RSHIFT" => Op::Rshift((*lhs).into(), (*rhs).parse::<i32>().unwrap_or_else(|_| panic!("Value expected"))),
//         _     => panic!("Unknown operation")
//     }
// }
