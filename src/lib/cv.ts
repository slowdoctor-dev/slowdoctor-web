export interface Publication {
  title: string;
  authors: string;
  journal: string;
  year: number;
  volume?: string;
  issue?: string;
  pages?: string;
  doi?: string;
  pubmed?: string;
}

export const publications: Publication[] = [
  {
    title:
      "Oncologic outcomes after immediate breast reconstruction following mastectomy: comparison of implant and flap using propensity score matching",
    authors:
      "Ha JH, Hong KY, Lee HB, Moon HG, Han W, Noh DY, Lim J, Yoon S, Chang H, Jin US",
    journal: "BMC Cancer",
    year: 2020,
    volume: "20",
    issue: "1",
    pages: "78",
    doi: "10.1186/s12885-020-6568-2",
    pubmed: "32000718",
  },
  {
    title:
      "Flap reconstruction of soft tissue defect after resecting a huge hemangioma of the nose",
    authors: "Lim J, Oh J, Eun S",
    journal: "Archives of Craniofacial Surgery",
    year: 2020,
    volume: "21",
    issue: "1",
    pages: "69-72",
    doi: "10.7181/acfs.2019.00668",
    pubmed: "32126625",
  },
  {
    title:
      "The reconstruction of the central tubercle in bilateral cleft lips: bilateral lateral mucosal advancement flap with reinforcement of the orbicularis oris muscle",
    authors: "Chung J, Lim J, Kim S, Koo Y",
    journal: "Annals of Plastic Surgery",
    year: 2019,
    volume: "83",
    issue: "6",
    pages: "655-659",
    doi: "10.1097/SAP.0000000000002012",
    pubmed: "31397684",
  },
  {
    title:
      "A case of multifocal primary cutaneous anaplastic large cell lymphoma managed without surgical treatment",
    authors: "Lim J, Park E, Eun S",
    journal: "Korean Journal of Head & Neck Oncology",
    year: 2019,
    volume: "35",
    issue: "2",
    pages: "77-81",
    doi: "10.21593/kjhno/2019.35.2.77",
  },
  {
    title:
      "Correlation between speech outcomes and the amount of maxillary advancement after orthognathic surgery in patients with cleft lip and palate",
    authors: "Chung J, Lim J, Park H, Yoo A, Kim S, Koo Y",
    journal: "The Journal of Craniofacial Surgery",
    year: 2019,
    volume: "30",
    issue: "6",
    pages: "1855-1858",
    doi: "10.1097/SCS.0000000000005623",
    pubmed: "31107383",
  },
  {
    title:
      "Portable ultrasonic surgery system for chronic wounds: a multicenter randomized controlled clinical trial and in vitro characterization",
    authors:
      "Pak C, Lim J, Kim BK, Kim H, Park S, Mun GH, Kim JT, Jeong JH, Heo CY",
    journal: "Journal of Wound Management and Research",
    year: 2019,
    volume: "15",
    issue: "1",
    pages: "5-10",
    doi: "10.22467/jwmr.2019.00584",
  },
  {
    title:
      "Immediate lower extremity reconstruction using an anterolateral thigh free flap with simultaneous interposition graft of descending branches of lateral circumflex femoral vessels",
    authors: "Lim J, Kwon H, Lee KM, Pak C",
    journal: "The International Journal of Lower Extremity Wounds",
    year: 2019,
    volume: "18",
    issue: "1",
    pages: "89-93",
    doi: "10.1177/1534734618819932",
    pubmed: "31064286",
  },
];
