//! JSON-LD structured data — ported from `src/lib/{schema,breadcrumbs}.ts`.

use crate::data::{
    doctor, Publication, AUTHOR_JOB_TITLE, AUTHOR_NAME, PRACTICE_NAME, PRACTICE_URL, SITE_URL,
};
use crate::types::BlogPostSummary;
use serde_json::{json, Value};

/// Person + Physician schema for the author. (was `generatePersonSchema`)
pub fn person_schema() -> Value {
    let d = doctor();
    json!({
        "@context": "https://schema.org",
        "@type": ["Person", "Physician"],
        "@id": d.id(),
        "name": d.name,
        "alternateName": d.alternate_name,
        "givenName": d.given_name,
        "familyName": d.family_name,
        "jobTitle": d.job_title,
        "description": d.description,
        "url": d.url,
        "image": d.image,
        "medicalSpecialty": d.medical_specialty,
        "alumniOf": d.alumni_of.iter().map(|s| json!({
            "@type": s.type_,
            "name": s.name,
        })).collect::<Vec<_>>(),
        "worksFor": {
            "@type": ["MedicalBusiness", "LocalBusiness"],
            "@id": d.works_for.id,
            "name": d.works_for.name,
            "url": d.works_for.url,
        },
        "memberOf": d.member_of.iter().map(|o| json!({
            "@type": "Organization",
            "name": o,
        })).collect::<Vec<_>>(),
        "hasCredential": {
            "@type": "EducationalOccupationalCredential",
            "credentialCategory": d.credential.category,
            "name": d.credential.name,
            "recognizedBy": {
                "@type": "Organization",
                "name": d.credential.recognized_by,
            },
        },
        "knowsAbout": d.knows_about,
        "sameAs": d.same_as,
    })
}

/// MedicalBusiness + LocalBusiness schema for the practice. (was `generatePracticeSchema`)
pub fn practice_schema() -> Value {
    let d = doctor();
    let w = &d.works_for;
    json!({
        "@context": "https://schema.org",
        "@type": ["MedicalBusiness", "LocalBusiness"],
        "@id": w.id,
        "name": w.name,
        "alternateName": w.alternate_name,
        "url": w.url,
        "telephone": w.phone,
        "address": {
            "@type": "PostalAddress",
            "streetAddress": w.address.street_address,
            "addressLocality": w.address.address_locality,
            "addressRegion": w.address.address_region,
            "postalCode": w.address.postal_code,
            "addressCountry": w.address.address_country,
        },
        "medicalSpecialty": d.medical_specialty,
        "founder": {
            "@type": "Person",
            "@id": d.id(),
            "name": d.name,
        },
    })
}

/// BreadcrumbList schema from (name, href) pairs. (was `buildBreadcrumbSchema`)
pub fn breadcrumb_schema(items: &[(&str, &str)]) -> Value {
    json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": items.iter().enumerate().map(|(i, (name, href))| json!({
            "@type": "ListItem",
            "position": i + 1,
            "name": name,
            "item": format!("{SITE_URL}{href}"),
        })).collect::<Vec<_>>(),
    })
}

pub fn blog_collection_schema(name: &str, description: &str, posts: &[BlogPostSummary]) -> Value {
    json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": name,
        "description": description,
        "url": format!("{SITE_URL}/blog"),
        "mainEntity": {
            "@type": "ItemList",
            "itemListElement": posts.iter().enumerate().map(|(i, post)| json!({
                "@type": "ListItem",
                "position": i + 1,
                "url": format!("{SITE_URL}/blog/{}", post.slug),
                "name": post.title,
            })).collect::<Vec<_>>(),
        },
    })
}

pub fn blog_posting_schema(summary: &BlogPostSummary, canonical: &str, image_url: &str) -> Value {
    let d = doctor();
    let mut article = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": summary.title,
        "description": summary.description,
        "datePublished": summary.date,
        "dateModified": summary.date,
        "url": canonical,
        "mainEntityOfPage": { "@type": "WebPage", "@id": canonical },
        "inLanguage": "en",
        "author": {
            "@type": "Person",
            "@id": d.id(),
            "name": AUTHOR_NAME,
            "url": format!("{SITE_URL}/cv"),
            "jobTitle": AUTHOR_JOB_TITLE,
            "worksFor": {
                "@type": "MedicalBusiness",
                "@id": d.works_for.id,
                "name": PRACTICE_NAME,
                "url": PRACTICE_URL,
            },
        },
        "publisher": { "@type": "Person", "@id": d.id(), "name": AUTHOR_NAME },
        "image": image_url,
    });
    if let Some(tags) = &summary.tags {
        if !tags.is_empty() {
            article["keywords"] = json!(tags.join(", "));
        }
    }
    article
}

pub fn scholarly_article_schema(publication: &Publication) -> Value {
    let d = doctor();
    let authors: Vec<_> = publication
        .authors
        .split(", ")
        .map(|name| {
            if name == "Lim J" {
                json!({ "@type": "Person", "name": name, "@id": d.id() })
            } else {
                json!({ "@type": "Person", "name": name })
            }
        })
        .collect();
    let mut schema = json!({
        "@context": "https://schema.org",
        "@type": "ScholarlyArticle",
        "headline": publication.title,
        "author": authors,
        "datePublished": publication.published_date.map(|s| s.to_string()).unwrap_or_else(|| publication.year.to_string()),
        "isPartOf": { "@type": "Periodical", "name": publication.journal },
    });
    if let Some(doi) = publication.doi {
        schema["url"] = json!(format!("https://doi.org/{doi}"));
        schema["identifier"] = json!({
            "@type": "PropertyValue",
            "propertyID": "DOI",
            "value": doi,
        });
    }
    schema
}
