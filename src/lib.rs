extern crate rand;

use rand::thread_rng;
use rand::Rng;
use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

/// Generates a Farsi compound name.
///
/// # Example
///
/// ```
/// use felfel::lib;
/// gen();
/// ```
#[wasm_bindgen]
pub fn gen() -> String {
    generate(0, ' ', false)
}

/// Generates a Farsi compound name with a numeric suffix.
///
/// # Example
///
/// ```
/// use felfel::lib;
/// gen_id();
/// ```
#[wasm_bindgen]
pub fn gen_id() -> String {
    generate(9999, '-', true)
}

/// Generates a Farsi compound name with a numeric suffix.
///
/// # Arguments
///
/// * `token_range` - Largest numeric suffix
/// * `delimiter` - The separator charachter
/// * `use_latin` - Map characters to Latin
///
/// # Example
///
/// ```
/// use felfel::lib;
/// generate(256, "~", true);
/// ```
fn generate(token_range: u32, delimiter: char, use_latin: bool) -> String {
    let mut rng = thread_rng();
    let a = rng.gen_range(0, NAMES.len());
    let mut out = String::from(if use_latin {NAMES[a].1} else {NAMES[a].0});

    if out.ends_with('ه') || out.ends_with('و') {
        out.push('‌');
        out.push('ی');
    } else if out.ends_with('e') || out.ends_with('i') || out.ends_with('o') {
        out.push_str("‌ye");
    } else if use_latin {
        out.push('e');
    }

    out.push(delimiter);

    let b = rng.gen_range(0, ADJECTIVES.len());
    out.push_str(if use_latin {ADJECTIVES[b].1} else {ADJECTIVES[b].0});

    if token_range > 0 {
        let roll = rng.gen_range(1, token_range);
        out.push(delimiter);
        out.push_str(&format!("{}", roll));
    }

    if !use_latin {
        out = out.chars()
        .map(|x| match x { 
            '0' => '۰',
            '1' => '۱',
            '2' => '۲',
            '3' => '۳',
            '4' => '۴',
            '5' => '۵',
            '6' => '۶',
            '7' => '۷',
            '8' => '۸',
            '9' => '۹',
            _ => x
        }).collect();
    }

    return out;   
}

const NAMES: &[(&str, &str)] = &[
    ("جوجه", "jooje"),
    ("پشه", "pashe"),
    ("سیب‌زمینی", "sibzamini"),
    ("پرنده", "parande"),
    ("حلزون", "halazoon"),
    ("خرگوش", "khargoosh"),
    ("اسب", "asb"),
    ("طوطی", "tooti"),
    ("گوزن", "gavazn"),
    ("آهو", "ahoo"),
    ("فیل", "fil"),
    ("مورچه", "moorche"),
    ("مورچه‌خوار", "moorchekhar"),
    ("موش", "moosh"),
    ("گربه", "gorbe"),
    ("سگ", "sag"),
    ("ماهی", "mahi"),
    ("مرغ‌دریایی", "morghedaryayi"),
    ("اسب آبی", "asbeabi"),
    ("فیل آبی", "fileabi"),
    ("زرافه", "zarafe"),
    ("اختاپوس", "okhtapoos"),
    ("دلقک", "dalghak"),
    ("کرگدن", "kargadan"),
    ("مهاجر", "mohajer"),
    ("مترسک", "matarsak"),
    ("شاپرک", "shaparak"),
    ("کبوتر", "kabootar"),
    ("پروانه", "parvane"),
    ("قورباغه", "ghoorbaghe"),
    ("شاهین", "shahin"),
    ("سیمرغ", "simorgh"),
    ("سنجاب", "sanjab"),
    ("یوز", "yooz"),
    ("قرقی", "gherghi"),
    ("تمساح", "temsah"),
    ("برنامه‌نویس", "barnamenevis"),
    ("هکر", "hacker"),
    ("دارکوب", "darkoob"),
    ("زنبور", "zanboor"),
    ("ساقی", "saaghi"),
    ("میمونک", "meymoonak"),
    ("نهنگ", "nahang"),
    ("آدمک", "adamak"),
    ("مسافر", "mosafer"),
    ("نابغه", "nabeghe"),
];

const ADJECTIVES: &[(&str, &str)] = &[
    ("گریان", "geryan"),
    ("خندان", "khandan"),
    ("هشیار", "hoshyar"),
    ("دیوانه", "divane"),
    ("پرمشغله", "pormashghale"),
    ("خوشفکر", "khoshfekr"),
    ("مبهوت", "mabhoot"),
    ("دوست‌داشتنی", "doostdashtani"),
    ("خسته", "khaste"),
    ("کنجکاو", "konjkav"),
    ("نگران", "negaran"),
    ("امیدوار", "omidvar"),
    ("مهربان", "mehraban"),
    ("تیزپا", "tizpa"),
    ("هوشمند", "hooshmand"),
    ("شاعر", "shaer"),
    ("درون", "daroon"),
    ("بی‌همتا", "bihamta"),
    ("بی‌نظیر", "binazir"),
    ("خجالتی", "khejalati"),
    ("سیری‌ناپذیر", "sirinapazir"),
    ("زیبا", "ziba"),
    ("اندیشمند", "andishmand"),
    ("صبور", "saboor"),
    ("سحرآمیز", "sehramiz"),
    ("بردبار", "bordbar"),
    ("دلسوز", "delsooz"),
    ("زودرنج", "zoodranj"),
    ("خوش‌مشرب", "khoshmashrab"),
    ("گشاده‌رو", "goshaderoo"),
    ("شجاع", "shoja"),
    ("دست و دلباز", "dashtodelbaz"),
    ("پرحرف", "porharf"),
    ("بانمک", "banamak"),
    ("چرب‌زبان", "charbzaban"),
    ("خوش‌شانس", "khoshshans"),
    ("خوش‌اقبال", "khosheghbal"),
    ("شوخ‌طبع", "shookhtab"),
    ("خوابآلو", "khabaloo"),
    ("خردمند", "kheradmand"),
    ("مشکوک", "mashkook"),
    ("بذله‌گو", "bazlegoo"),
    ("جذاب", "jazab"),
    ("حواس‌پرت", "havaspart"),
    ("غمگین", "ghamgin"),
    ("باوفا", "bavafa"),
    ("وفادار", "vafadar"),
    ("مؤمن", "momen"),
    ("فناناپذیر", "fananapazir"),
    ("قدیمی", "ghadimi"),
    ("خوشنام", "khoshnam"),
    ("جسور", "jasoor"),
    ("خستگی‌ناپذیر", "khasteginapazir"),
    ("ریزنقش", "riznaqsh"),
    ("بلندپرواز", "bolandparvaz"),
    ("بی‌پروا", "biparva"),
    ("تسلیم‌ناپذیر", "taslimnapazir"),
    ("سمج", "semej"),
    ("بازیگوش", "bazigoosh"),
];
