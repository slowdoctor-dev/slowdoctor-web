//! Static site data — ported from `src/data/*` and `src/lib/{config,links,cv}.ts`.
//! Single source of truth for the doctor profile, site config, links, and CV.

/// Canonical site URL (was `src/data/site.json`).
pub const SITE_URL: &str = "https://slowdoctor.dev";

// ---------------------------------------------------------------------------
// Doctor profile (was src/data/doctor.ts)
// ---------------------------------------------------------------------------

pub struct AlumniEntry {
    pub name: &'static str,
    pub type_: &'static str,
}

pub struct Address {
    pub street_address: &'static str,
    pub address_locality: &'static str,
    pub address_region: &'static str,
    pub postal_code: &'static str,
    pub address_country: &'static str,
}

pub struct Credential {
    pub name: &'static str,
    pub category: &'static str,
    pub recognized_by: &'static str,
}

pub struct WorksFor {
    pub id: &'static str,
    pub name: &'static str,
    pub alternate_name: &'static str,
    pub url: &'static str,
    pub location: &'static str,
    pub address: Address,
    pub phone: &'static str,
}

pub struct Doctor {
    pub name: &'static str,
    pub alternate_name: &'static str,
    pub given_name: &'static str,
    pub family_name: &'static str,
    pub title: &'static str,
    pub job_title: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub image: String,
    pub alumni_of: Vec<AlumniEntry>,
    pub credential: Credential,
    pub member_of: Vec<&'static str>,
    pub works_for: WorksFor,
    pub knows_about: Vec<&'static str>,
    pub medical_specialty: &'static str,
    pub same_as: Vec<&'static str>,
}

impl Doctor {
    /// Stable JSON-LD `@id` for the person node (`{url}/#person`).
    pub fn id(&self) -> String {
        format!("{}/#person", self.url)
    }
}

pub fn doctor() -> Doctor {
    Doctor {
        name: "Joonho Lim",
        alternate_name: "임준호",
        given_name: "Joonho",
        family_name: "Lim",
        title: "Joonho Lim, M.D.",
        job_title: "Board-Certified Plastic Surgeon",
        description: "Board-certified plastic surgeon specializing in slow-aging, scar treatment, and natural eye surgery. Founder of LEAD Plastic Surgery Clinic in Seoul.",
        url: SITE_URL,
        image: format!("{SITE_URL}/images/profile.jpg"),
        alumni_of: vec![
            AlumniEntry { name: "Seoul Science High School", type_: "EducationalOrganization" },
            AlumniEntry { name: "Seoul National University College of Medicine", type_: "CollegeOrUniversity" },
        ],
        credential: Credential {
            name: "Plastic Surgery Specialist",
            category: "Board Certification",
            recognized_by: "Korean Medical Association",
        },
        member_of: vec![
            "Korean Society of Plastic and Reconstructive Surgeons",
            "Korean Society for Aesthetic Plastic Surgery",
        ],
        works_for: WorksFor {
            id: "https://leadps.co.kr/#organization",
            name: "LEAD Plastic Surgery Clinic",
            alternate_name: "리드성형외과의원",
            url: "https://leadps.co.kr",
            location: "Dogok-dong, Gangnam-gu, Seoul, South Korea",
            address: Address {
                street_address: "21 Eonju-ro 30-gil, B101-31-1",
                address_locality: "Gangnam-gu",
                address_region: "Seoul",
                postal_code: "06292",
                address_country: "KR",
            },
            phone: "+82-2-6953-3231",
        },
        knows_about: vec![
            "Anti-aging medicine",
            "Non-surgical facial rejuvenation",
            "Ultrasound skin lifting",
            "Radiofrequency skin tightening",
            "Dermal filler",
            "Botulinum toxin injection",
            "Scar treatment and revision",
            "Keloid management",
            "Non-incisional blepharoplasty",
            "Lower blepharoplasty with fat repositioning",
            "Endoscopic brow lift",
        ],
        medical_specialty: "PlasticSurgery",
        same_as: vec![
            "https://orcid.org/0000-0002-4556-1536",
            "https://scholar.google.com/citations?user=i_e44lEAAAAJ",
            "https://www.researchgate.net/profile/Joonho-Lim",
            "https://www.linkedin.com/in/slowdoctor/",
            "https://github.com/slowdoctor-dev",
            "https://www.youtube.com/@slowdoctor",
            "https://www.instagram.com/slowdoctor_/",
            "https://www.threads.net/@slowdoctor_",
            "https://www.tiktok.com/@slowdoctor_",
            "https://blog.naver.com/plastic_talks",
        ],
    }
}

// ---------------------------------------------------------------------------
// Site config (was src/lib/config.ts)
// ---------------------------------------------------------------------------

pub const SITE_NAME: &str = "slowdoctor.dev";
pub const SITE_TITLE: &str = "Joonho Lim - Plastic Surgeon & Engineer";
/// Title template; `%s` is replaced with the page title.
pub const SITE_TITLE_TEMPLATE: &str = "%s | Joonho Lim";
pub const OG_IMAGE: &str = "/og-default.png";

pub const AUTHOR_NAME: &str = "Joonho Lim";
pub const AUTHOR_KOREAN: &str = "임준호";
pub const AUTHOR_CREDENTIALED_NAME: &str = "Joonho Lim, M.D.";
pub const AUTHOR_JOB_TITLE: &str = "Board-Certified Plastic Surgeon";

pub const DESCRIPTION_FULL: &str = "Board-certified plastic surgeon and engineer specializing in slow-aging, scar treatment, and blepharoplasty. Practicing in Gangnam, Seoul, with a focus on long-term results over quick fixes.";
pub const DESCRIPTION_BRIEF: &str = "Board-certified plastic surgeon and engineer, practicing in Gangnam, Seoul.";

pub const PRACTICE_NAME: &str = "LEAD Plastic Surgery Clinic";
pub const PRACTICE_LOCATION: &str = "Dogok-dong, Gangnam-gu, Seoul, South Korea";
pub const PRACTICE_PHONE: &str = "+82-2-6953-3231";

// ---------------------------------------------------------------------------
// Links (was src/lib/links.ts)
// ---------------------------------------------------------------------------

pub struct SocialLink {
    pub label: &'static str,
    pub url: &'static str,
    pub handle: &'static str,
}

pub struct MedicalLink {
    pub label: &'static str,
    pub url: &'static str,
    pub detail: &'static str,
}

pub const GITHUB_URL: &str = "https://github.com/slowdoctor-dev";
pub const REPO_URL: &str = "https://github.com/slowdoctor-dev/slowdoctor-web";
pub const PRACTICE_URL: &str = "https://leadps.co.kr";

pub fn social_links() -> Vec<SocialLink> {
    vec![
        SocialLink { label: "YouTube", url: "https://www.youtube.com/@slowdoctor", handle: "@slowdoctor" },
        SocialLink { label: "Instagram", url: "https://www.instagram.com/slowdoctor_/", handle: "@slowdoctor_" },
        SocialLink { label: "Threads", url: "https://www.threads.net/@slowdoctor_", handle: "@slowdoctor_" },
        SocialLink { label: "TikTok", url: "https://www.tiktok.com/@slowdoctor_", handle: "@slowdoctor_" },
        SocialLink { label: "Naver Blog", url: "https://blog.naver.com/plastic_talks", handle: "plastic_talks" },
    ]
}

pub fn medical_links() -> Vec<MedicalLink> {
    vec![
        MedicalLink { label: "ORCID", url: "https://orcid.org/0000-0002-4556-1536", detail: "0000-0002-4556-1536" },
        MedicalLink { label: "Google Scholar", url: "https://scholar.google.com/citations?user=i_e44lEAAAAJ", detail: "Publications" },
        MedicalLink { label: "ResearchGate", url: "https://www.researchgate.net/profile/Joonho-Lim", detail: "Profile" },
        MedicalLink { label: "LinkedIn", url: "https://www.linkedin.com/in/slowdoctor/", detail: "@slowdoctor" },
    ]
}

// ---------------------------------------------------------------------------
// CV publications (was src/lib/cv.ts)
// ---------------------------------------------------------------------------

pub struct Publication {
    pub title: &'static str,
    pub authors: &'static str,
    pub journal: &'static str,
    pub year: u32,
    pub published_date: Option<&'static str>,
    pub volume: Option<&'static str>,
    pub issue: Option<&'static str>,
    pub pages: Option<&'static str>,
    pub doi: Option<&'static str>,
    pub pubmed: Option<&'static str>,
}

pub fn publications() -> Vec<Publication> {
    vec![
        Publication {
            title: "Oncologic outcomes after immediate breast reconstruction following mastectomy: comparison of implant and flap using propensity score matching",
            authors: "Ha JH, Hong KY, Lee HB, Moon HG, Han W, Noh DY, Lim J, Yoon S, Chang H, Jin US",
            journal: "BMC Cancer",
            year: 2020,
            published_date: Some("2020-01-30"),
            volume: Some("20"), issue: Some("1"), pages: Some("78"),
            doi: Some("10.1186/s12885-020-6568-2"), pubmed: Some("32000718"),
        },
        Publication {
            title: "Flap reconstruction of soft tissue defect after resecting a huge hemangioma of the nose",
            authors: "Lim J, Oh J, Eun S",
            journal: "Archives of Craniofacial Surgery",
            year: 2020,
            published_date: Some("2020-02-20"),
            volume: Some("21"), issue: Some("1"), pages: Some("69-72"),
            doi: Some("10.7181/acfs.2019.00668"), pubmed: Some("32126625"),
        },
        Publication {
            title: "Neuroma of the dorsal rami in the back and its surgical treatment: a case report",
            authors: "Lim J, Eun S",
            journal: "Archives of Hand and Microsurgery",
            year: 2020,
            published_date: Some("2020-03-01"),
            volume: Some("25"), issue: Some("1"), pages: Some("67-70"),
            doi: Some("10.12790/ahm.19.0047"), pubmed: None,
        },
        Publication {
            title: "The reconstruction of the central tubercle in bilateral cleft lips: bilateral lateral mucosal advancement flap with reinforcement of the orbicularis oris muscle",
            authors: "Chung J, Lim J, Kim S, Koo Y",
            journal: "Annals of Plastic Surgery",
            year: 2019,
            published_date: Some("2019-08-08"),
            volume: Some("83"), issue: Some("6"), pages: Some("655-659"),
            doi: Some("10.1097/SAP.0000000000002012"), pubmed: Some("31397684"),
        },
        Publication {
            title: "A case of multifocal primary cutaneous anaplastic large cell lymphoma managed without surgical treatment",
            authors: "Lim J, Park E, Eun S",
            journal: "Korean Journal of Head & Neck Oncology",
            year: 2019,
            published_date: Some("2019-11-30"),
            volume: Some("35"), issue: Some("2"), pages: Some("77-81"),
            doi: Some("10.21593/kjhno/2019.35.2.77"), pubmed: None,
        },
        Publication {
            title: "Correlation between speech outcomes and the amount of maxillary advancement after orthognathic surgery in patients with cleft lip and palate",
            authors: "Chung J, Lim J, Park H, Yoo A, Kim S, Koo Y",
            journal: "The Journal of Craniofacial Surgery",
            year: 2019,
            published_date: Some("2019-09-01"),
            volume: Some("30"), issue: Some("6"), pages: Some("1855-1858"),
            doi: Some("10.1097/SCS.0000000000005623"), pubmed: Some("31107383"),
        },
        Publication {
            title: "Portable ultrasonic surgery system for chronic wounds: a multicenter randomized controlled clinical trial and in vitro characterization",
            authors: "Pak C, Lim J, Kim BK, Kim H, Park S, Mun GH, Kim JT, Jeong JH, Heo CY",
            journal: "Journal of Wound Management and Research",
            year: 2019,
            published_date: Some("2019-03-31"),
            volume: Some("15"), issue: Some("1"), pages: Some("5-10"),
            doi: Some("10.22467/jwmr.2019.00584"), pubmed: None,
        },
        Publication {
            title: "Immediate lower extremity reconstruction using an anterolateral thigh free flap with simultaneous interposition graft of descending branches of lateral circumflex femoral vessels",
            authors: "Lim J, Kwon H, Lee KM, Pak C",
            journal: "The International Journal of Lower Extremity Wounds",
            year: 2019,
            published_date: Some("2018-12-28"),
            volume: Some("18"), issue: Some("1"), pages: Some("89-93"),
            doi: Some("10.1177/1534734618819932"), pubmed: Some("31064286"),
        },
    ]
}
