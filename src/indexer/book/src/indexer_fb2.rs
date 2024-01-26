use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use fb2::{Author, FictionBook, Genre};

use harana_common::anyhow::Result;
use harana_common::hashbrown::HashSet;
use harana_common::serde::{self, Deserialize, Serialize};
use harana_common::serde_json;
use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;
use harana_indexer_core::tokenizer::tokenize;

#[derive(Deserialize, Serialize)]
#[serde(crate = "self::serde")]
pub struct Fb2Metadata {
    title: String,
    authors: HashSet<String>,
    publisher: Option<String>,
    isbn: Option<String>,
    publication_date: Option<String>,
    genres: HashSet<String>
}

pub struct IndexerFb2;

impl Indexer for IndexerFb2 {

    fn index(&self, path: PathBuf) -> Result<IndexResult> {
        let file = File::open(path.clone())?;
        let reader = BufReader::new(file);
        let book: FictionBook = quick_xml::de::from_reader(reader)?;

        let mut primary_tokens = HashSet::<String>::new();
        let mut secondary_tokens = HashSet::<String>::new();

        let title_info = book.description.title_info;
        let publish_info = book.description.publish_info;

        let title = title_info.book_title.value;
        let publication_date = title_info.date.and_then(|d| d.display_date);
        let keywords = title_info.keywords.map(|v| v.value).unwrap_or("".to_string());

        let publisher = publish_info.clone().and_then(|p| p.publisher).map(|p| p.value);
        let isbn = publish_info.clone().and_then(|p| p.isbn).map(|p| p.value);


        let mut genres = HashSet::new();
        title_info.genres.into_iter().for_each(|genre| {
            let pretty: Vec<String> = pretty_genre(genre.value).into_iter().map(|g| g.to_string()).collect();
            genres.extend(pretty);
        });

        let mut authors = HashSet::new();
        title_info.authors.into_iter().for_each(|author| {
            match author {
                Author::Verbose(v) => {
                    authors.insert(
                        match v.middle_name {
                            None => format!("{} {}", v.first_name.value, v.last_name.value),
                            Some(m) => format!("{} {} {}", v.first_name.value, m.value, v.last_name.value)
                        }
                    );
                }
                Author::Anonymous(a) => {
                    if a.nickname.is_some() {
                        authors.insert(a.nickname.unwrap().value);
                    }
                }
            }
        });

        primary_tokens.extend(tokenize(title.as_str()));
        primary_tokens.extend(tokenize(keywords.as_str()));
        primary_tokens.extend(authors.clone());
        primary_tokens.extend(genres.clone());

        let metadata = Fb2Metadata {
            title,
            authors,
            publisher,
            isbn,
            publication_date,
            genres
        };

        Ok(IndexResult {
            path,
            title: None,
            description: None,
            author: None,
            primary_tokens,
            secondary_tokens,
            metadata: serde_json::to_value(&metadata)?
        })
    }
}

fn pretty_genre(genre: Genre) -> Vec<&'static str> {
    match genre {
        Genre::Accounting =>                    vec!("Accounting"),
        Genre::AdvAnimal =>                     vec!("Animal"),
        Genre::AdvGeo =>                        vec!("Adventure", "Geography"),
        Genre::AdvHistory =>                    vec!("Adventure", "History"),
        Genre::AdvIndian =>                     vec!("Adventure", "Indian"),
        Genre::AdvMaritime =>                   vec!("Adventure", "Maritime"),
        Genre::AdvWestern =>                    vec!("Adventure", "Western"),
        Genre::Adventure =>                     vec!("Adventure"),
        Genre::Antique =>                       vec!("Antique"),
        Genre::AntiqueAnt =>                    vec!("Antique"),
        Genre::AntiqueEast =>                   vec!("Antique", "East"),
        Genre::AntiqueEuropean =>               vec!("Antique", "European"),
        Genre::AntiqueMyths =>                  vec!("Antique", "Myths"),
        Genre::AntiqueRussian =>                vec!("Antique", "Russian"),
        Genre::AphorismQuote =>                 vec!("Aphorism", "Quote"),
        Genre::Architecture =>                  vec!("Architecture"),
        Genre::ArchitectureBook =>              vec!("Architecture"),
        Genre::AutoRegulations =>               vec!("Regulations"),
        Genre::Banking =>                       vec!("Banking"),
        Genre::BeginningAuthors =>              vec!("Beginning", "Authors"),
        Genre::Business =>                      vec!("Business"),
        Genre::ChildAdv =>                      vec!("Child", "Adventure"),
        Genre::ChildDet =>                      vec!("Child", "Detective"),
        Genre::ChildEducation =>                vec!("Child", "Education"),
        Genre::ChildProse =>                    vec!("Child", "Prose"),
        Genre::ChildSf =>                       vec!("Child", "Science Fiction"),
        Genre::ChildTale =>                     vec!("Child", "Tale"),
        Genre::ChildVerse =>                    vec!("Child", "Verse"),
        Genre::Children =>                      vec!("Children"),
        Genre::CinemaTheatre =>                 vec!("Cinema", "Theatre"),
        Genre::CityFantasy =>                   vec!("City", "Fantasy"),
        Genre::CompDb =>                        vec!("Computer", "Database"),
        Genre::CompHard =>                      vec!("Computer", "Hardware"),
        Genre::CompOsnet =>                     vec!("Computer", "Networking"),
        Genre::CompProgramming =>               vec!("Computer", "Programming"),
        Genre::CompSoft =>                      vec!("Computer", "Software"),
        Genre::CompWww =>                       vec!("Computer", "Web"),
        Genre::Computers =>                     vec!("Computers"),
        Genre::Design =>                        vec!("Design"),
        Genre::DetAction =>                     vec!("Detective", "Action"),
        Genre::DetClassic =>                    vec!("Detective", "Classic"),
        Genre::DetCrime =>                      vec!("Detective", "Crime"),
        Genre::DetEspionage =>                  vec!("Detective", "Espionage"),
        Genre::DetHard =>                       vec!("Detective", "Hard"),
        Genre::DetHistory =>                    vec!("Detective", "History"),
        Genre::DetIrony =>                      vec!("Detective", "Irony"),
        Genre::DetManiac =>                     vec!("Detective", "Maniac"),
        Genre::DetPolice =>                     vec!("Detective", "Police"),
        Genre::DetPolitical =>                  vec!("Detective", "Political"),
        Genre::Detective =>                     vec!("Detective", "Detective"),
        Genre::DragonFantasy =>                 vec!("Dragon", "Fantasy"),
        Genre::Dramaturgy =>                    vec!("Dramaturgy"),
        Genre::Economics =>                     vec!("Economics"),
        Genre::EconomicsRef =>                  vec!("Economics", "Reference"),
        Genre::EntertHumor =>                   vec!("Entertainment", "Humor"),
        Genre::Essays =>                        vec!("Essays"),
        Genre::FantasyFight =>                  vec!("Fantasy", "Fight"),
        Genre::ForeignAction =>                 vec!("Foreign", "Action"),
        Genre::ForeignAdventure =>              vec!("Foreign", "Adventure"),
        Genre::ForeignAntique =>                vec!("Foreign", "Antique"),
        Genre::ForeignBusiness =>               vec!("Foreign", "Business"),
        Genre::ForeignChildren =>               vec!("Foreign", "Children"),
        Genre::ForeignContemporary =>           vec!("Foreign", "Contemporary"),
        Genre::ForeignContemporaryLit =>        vec!("Foreign", "Contemporary", "Literature"),
        Genre::ForeignDetective =>              vec!("Foreign", "Detective"),
        Genre::ForeignDramaturgy =>             vec!("Foreign", "Dramaturgy"),
        Genre::ForeignEdu =>                    vec!("Foreign", "Education"),
        Genre::ForeignFantasy =>                vec!("Foreign", "Fantasy"),
        Genre::ForeignHome =>                   vec!("Foreign", "Home"),
        Genre::ForeignHumor =>                  vec!("Foreign", "Humor"),
        Genre::ForeignLanguage =>               vec!("Foreign", "Language"),
        Genre::ForeignLove =>                   vec!("Foreign", "Love"),
        Genre::ForeignNovel =>                  vec!("Foreign", "Novel"),
        Genre::ForeignPoetry =>                 vec!("Foreign", "Poetry"),
        Genre::ForeignProse =>                  vec!("Foreign", "Prose"),
        Genre::ForeignPsychology =>             vec!("Foreign", "Psychology"),
        Genre::ForeignPublicism =>              vec!("Foreign", "Publicism"),
        Genre::ForeignReligion =>               vec!("Foreign", "Religion"),
        Genre::ForeignSf =>                     vec!("Foreign", "Science Fiction"),
        Genre::GeoGuides =>                     vec!("Geography", "Guides"),
        Genre::GeographyBook =>                 vec!("Geography"),
        Genre::GlobalEconomy =>                 vec!("Global Economy"),
        Genre::HealthPsy =>                     vec!("Health", "Psychology"),
        Genre::HistoricalFantasy =>             vec!("Historical", "Fantasy"),
        Genre::Home =>                          vec!("Home"),
        Genre::HomeCooking =>                   vec!("Home", "Cooking"),
        Genre::HomeCrafts =>                    vec!("Home", "Crafts"),
        Genre::HomeDiy =>                       vec!("Home", "DIY"),
        Genre::HomeEntertain =>                 vec!("Home", "Entertainment"),
        Genre::HomeHealth =>                    vec!("Home", "Health"),
        Genre::HomePets =>                      vec!("Home", "Pets"),
        Genre::HomeSex =>                       vec!("Home", "Sex"),
        Genre::HomeSport =>                     vec!("Home", "Sport"),
        Genre::HorrorFantasy =>                 vec!("Horror", "Fantasy"),
        Genre::HorrorVampires =>                vec!("Horror", "Vampires"),
        Genre::Humor =>                         vec!("Humor"),
        Genre::HumorAnecdote =>                 vec!("Humor", "Anecdote"),
        Genre::HumorFantasy =>                  vec!("Humor", "Fantasy"),
        Genre::HumorProse =>                    vec!("Humor", "Prose"),
        Genre::HumorVerse =>                    vec!("Humor", "Verse"),
        Genre::Industries =>                    vec!("Industries"),
        Genre::JobHunting =>                    vec!("Job Hunting"),
        Genre::Literature18 =>                  vec!("Literature"),
        Genre::Literature19 =>                  vec!("Literature"),
        Genre::Literature20 =>                  vec!("Literature"),
        Genre::LiteratureAdv =>                 vec!("Literature"),
        Genre::LiteratureFairy =>               vec!("Literature", "Fairy"),
        Genre::LiteratureHistory =>             vec!("Literature", "History"),
        Genre::Love =>                          vec!("Love"),
        Genre::LoveContemporary =>              vec!("Love", "Contemporary"),
        Genre::LoveDetective =>                 vec!("Love", "Detective"),
        Genre::LoveErotica =>                   vec!("Love", "Erotica"),
        Genre::LoveFantasy =>                   vec!("Love", "Fantasy"),
        Genre::LoveHistory =>                   vec!("Love", "History"),
        Genre::LoveSf =>                        vec!("Love", "Science Fiction"),
        Genre::LoveShort =>                     vec!("Love", "Short"),
        Genre::MagicianBook =>                  vec!("Magician"),
        Genre::Management =>                    vec!("Management"),
        Genre::Marketing =>                     vec!("Marketing"),
        Genre::MilitarySpecial =>               vec!("Military", "Special"),
        Genre::MusicDancing =>                  vec!("Music", "Dancing"),
        Genre::Narrative =>                     vec!("Narrative"),
        Genre::Newspapers =>                    vec!("Newspapers"),
        Genre::NonfBiography =>                 vec!("Nonfiction", "Biography"),
        Genre::NonfCriticism =>                 vec!("Nonfiction", "Criticism"),
        Genre::NonfPublicism =>                 vec!("Nonfiction", "Publicism"),
        Genre::Nonfiction =>                    vec!("Nonfiction"),
        Genre::NonfictionPolitics =>            vec!("Nonfiction", "Politics"),
        Genre::NonfictionSociology =>           vec!("Nonfiction", "Sociology"),
        Genre::OrgBehavior =>                   vec!("Organisational", "Behavior"),
        Genre::PaperWork =>                     vec!("Paperwork"),
        Genre::PedagogyBook =>                  vec!("Pedagogy", "Book"),
        Genre::Periodic =>                      vec!("Periodic"),
        Genre::PersonalFinance =>               vec!("Personal Finance"),
        Genre::Poetry =>                        vec!("Poetry"),
        Genre::PopularBusiness =>               vec!("Popular", "Business"),
        Genre::Prose =>                         vec!("Prose"),
        Genre::ProseClassic =>                  vec!("Prose", "Classic"),
        Genre::ProseContemporary =>             vec!("Prose", "Contemporary"),
        Genre::ProseCounter =>                  vec!("Prose", "Counter"),
        Genre::ProseHistory =>                  vec!("Prose", "History"),
        Genre::ProseMilitary =>                 vec!("Prose", "Military"),
        Genre::ProseRusClassic =>               vec!("Prose", "Russian Classic"),
        Genre::PsyAlassic =>                    vec!("Psychology"),
        Genre::PsyChilds =>                     vec!("Psychology", "Children"),
        Genre::PsyGeneric =>                    vec!("Psychology", "Generic"),
        Genre::PsyPersonal =>                   vec!("Psychology", "Personal"),
        Genre::PsySexAndFamily =>               vec!("Psychology", "Sex", "Family"),
        Genre::PsySocial =>                     vec!("Psychology", "Social"),
        Genre::PsyTheraphy =>                   vec!("Psychology", "Therapy"),
        Genre::RealEstate =>                    vec!("Real Estate"),
        Genre::RefDict =>                       vec!("Reference", "Dictionary"),
        Genre::RefEncyc =>                      vec!("Reference", "Encyclopaedia"),
        Genre::RefGuide =>                      vec!("Reference", "Guide"),
        Genre::Reference =>                     vec!("Reference"),
        Genre::Religion =>                      vec!("Religion"),
        Genre::ReligionBudda =>                 vec!("Religion", "Buddah"),
        Genre::ReligionEsoterics =>             vec!("Religion", "Esoterics"),
        Genre::ReligionSelf =>                  vec!("Religion", "Self"),
        Genre::RomanceContemporary =>           vec!("Romance", "Contemporary"),
        Genre::RomanceFantasy =>                vec!("Romance", "Fantasy"),
        Genre::RomanceHistorical =>             vec!("Romance", "Historical"),
        Genre::RomanceRomanticSuspense =>       vec!("Romance", "Suspense"),
        Genre::RomanceSf =>                     vec!("Romance", "Science Fiction"),
        Genre::RussianContemporary =>           vec!("Russian", "Contemporary"),
        Genre::RussianFantasy =>                vec!("Russian", "Fantasy"),
        Genre::SciBiology =>                    vec!("Science", "Biology"),
        Genre::SciBusiness =>                   vec!("Science", "Business"),
        Genre::SciChem =>                       vec!("Science", "Chemistry"),
        Genre::SciCosmos =>                     vec!("Science", "Cosmos"),
        Genre::SciCulture =>                    vec!("Science", "Culture"),
        Genre::SciEconomy =>                    vec!("Science", "Economy"),
        Genre::SciHistory =>                    vec!("Science", "History"),
        Genre::SciJuris =>                      vec!("Science", "Juris"),
        Genre::SciLinguistic =>                 vec!("Science", "Linguistic"),
        Genre::SciMath =>                       vec!("Science", "Math"),
        Genre::SciMedicine =>                   vec!("Science", "Medicine"),
        Genre::SciPhilology =>                  vec!("Science", "Philology"),
        Genre::SciPhilosophy =>                 vec!("Science", "Philosophy"),
        Genre::SciPhys =>                       vec!("Science", "Physics"),
        Genre::SciPolitics =>                   vec!("Science", "Politics"),
        Genre::SciPsychology =>                 vec!("Science", "Psychology"),
        Genre::SciRadio =>                      vec!("Science", "Radio"),
        Genre::SciReligion =>                   vec!("Science", "Religion"),
        Genre::SciState =>                      vec!("Science", "State"),
        Genre::SciTech =>                       vec!("Science", "Technology"),
        Genre::SciTransport =>                  vec!("Science", "Transport"),
        Genre::Science =>                       vec!("Science"),
        Genre::Sf =>                            vec!("Science Fiction"),
        Genre::SfAction =>                      vec!("Science Fiction", "Action"),
        Genre::SfCyberPunk =>                   vec!("Science Fiction", "Cyberpunk"),
        Genre::SfCyberpunk =>                   vec!("Science Fiction", "Cyberpunk"),
        Genre::SfDetective =>                   vec!("Science Fiction", "Detective"),
        Genre::SfEpic =>                        vec!("Science Fiction", "Epic"),
        Genre::SfFantasy =>                     vec!("Science Fiction", "Fantasy"),
        Genre::SfFantasyCity =>                 vec!("Science Fiction", "Fantasy City"),
        Genre::SfHeroic =>                      vec!("Science Fiction", "Heroic"),
        Genre::SfHistory =>                     vec!("Science Fiction", "History"),
        Genre::SfHorror =>                      vec!("Science Fiction", "Horror"),
        Genre::SfHumor =>                       vec!("Science Fiction", "Humor"),
        Genre::SfMystic =>                      vec!("Science Fiction", "Mystic"),
        Genre::SfPostapocalyptic =>             vec!("Science Fiction", "Postapocalyptic"),
        Genre::SfSocial =>                      vec!("Science Fiction", "Social"),
        Genre::SfSpace =>                       vec!("Science Fiction", "Space"),
        Genre::SfWriting =>                     vec!("Science Fiction", "Writing"),
        Genre::ShortStory =>                    vec!("Short Story"),
        Genre::Sketch =>                        vec!("Sketch"),
        Genre::SmallBusiness =>                 vec!("Small Business"),
        Genre::SociologyBook =>                 vec!("Sociology"),
        Genre::Stock =>                         vec!("Stock"),
        Genre::Thriller =>                      vec!("Thriller"),
        Genre::ThrillerMystery =>               vec!("Mystery"),
        Genre::UpbringingBook =>                vec!("Upbringing"),
        Genre::VampireBook =>                   vec!("Vampire"),
        Genre::VisualArts =>                    vec!("Visual Arts"),
        _ =>                                    vec!()
    }
}

#[cfg(test)]
mod tests {
    use harana_common::{itertools, tokio};
    use harana_indexer_core::index_result::IndexResult;
use harana_indexer_core::indexer::Indexer;

    use crate::indexer_fb2::IndexerFb2;

    #[tokio::test]
    async fn test_indexing() {
        let file = IndexResult {
            path: "".to_string().into(),
            title: None,
            description: None,
            author: None,
            primary_tokens: Default::default(),
            secondary_tokens: Default::default(),
            metadata: Default::default(),
        };

        let indexed_file = IndexerFb2.index("../../../test_files/Sample1.fb2").unwrap();
        println!("Primary Tokens: {}", itertools::join(&indexed_file.primary_tokens, ", "));
        println!("Metadata: {}", indexed_file.metadata.to_string());
    }
}