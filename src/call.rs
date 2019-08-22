use chrono::prelude::*;
use std::collections::HashMap;

const BASEURL: &str = "https://www.boardgamegeek.com/xmlapi2/";

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ThingType {
    Boardgame,
    BoardgameAccessory,
    BoardgameExpansion,
    Videogame,
    RPGItem,
    RPGIssue
}

impl ThingType {
    pub fn as_str(&self) -> &str {
        match self {
            &ThingType::Boardgame => "boardgame",
            &ThingType::BoardgameAccessory => "boardgameaccessory",
            &ThingType::BoardgameExpansion => "boardgameexpansion",
            &ThingType::Videogame => "videogame",
            &ThingType::RPGItem => "rpgitem",
            &ThingType::RPGIssue => "rpgissue"
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CollectionArgument {
    Ownership,
    Rated,
    Played,
    Commented,
    MarkedForTrade,
    WantedForTrade,
    Wishlisted,
    Preordered,
    WantToPlay,
    WantToBuy,
    PreviouslyOwned,
    HasParts,
    WantParts
}

impl CollectionArgument {
    pub fn as_str(&self) -> &str {
        match self {
            &CollectionArgument::Ownership => "own",
            &CollectionArgument::Rated => "rated",
            &CollectionArgument::Played => "played",
            &CollectionArgument::Commented => "comment",
            &CollectionArgument::MarkedForTrade => "trade",
            &CollectionArgument::WantedForTrade => "want",
            &CollectionArgument::Wishlisted => "wishlist",
            &CollectionArgument::Preordered => "preordered",
            &CollectionArgument::WantToPlay => "wanttoplay",
            &CollectionArgument::WantToBuy => "wanttobuy",
            &CollectionArgument::PreviouslyOwned => "prevowned",
            &CollectionArgument::HasParts => "hasparts",
            &CollectionArgument::WantParts => "wantparts",

        }
    }
}

pub struct Thing {
    action: String,
    id: Vec<usize>,
    r#type: Vec<ThingType>,
    versions: bool,
    videos: bool,
    stats: bool,
    historical: bool,
    marketplace: bool,
    ratingcomments: bool,
    page: usize,
    pagesize: usize
}

impl Thing {
    pub fn new() -> Thing {
        Thing {
            action: "thing".to_string(),
            id: Vec::new(),
            r#type: Vec::new(),
            versions: true,
            videos: true,
            stats: true,
            historical: true,
            marketplace: true,
            ratingcomments: true,
            page: 1,
            pagesize: 10
        }
    }

    pub fn with_id<'a>(&'a mut self, id: usize) -> &'a mut Thing {
        self.id.push(id);
        self.id.sort();
        self.id.dedup();
        self
    }

    pub fn with_type<'a>(&'a mut self, thingtype: ThingType) -> &'a mut Thing {
        self.r#type.push(thingtype);
        self.r#type.sort();
        self.r#type.dedup();
        self
    }

    pub fn no_versions<'a>(&'a mut self) -> &'a mut Thing {
        self.versions = false;
        self
    }

    pub fn no_videos<'a>(&'a mut self) -> &'a mut Thing {
        self.videos = false;
        self
    }

    pub fn no_stats<'a>(&'a mut self) -> &'a mut Thing {
        self.stats = false;
        self
    }

    pub fn no_historical<'a>(&'a mut self) -> &'a mut Thing {
        self.historical = false;
        self
    }

    pub fn no_marketplace<'a>(&'a mut self) -> &'a mut Thing {
        self.marketplace = false;
        self
    }

    pub fn no_ratingcomments<'a>(&'a mut self) -> &'a mut Thing {
        self.ratingcomments = false;
        self
    }

    pub fn page<'a>(&'a mut self, page: usize) -> &'a mut Thing {
        self.page = page;
        self
    }

    pub fn pagesize<'a>(&'a mut self, pagesize: usize) -> &'a mut Thing {
        self.pagesize = pagesize;
        self
    }

    pub fn to_string<'a>(&'a mut self) -> String {
        let id_str: Vec<String> = self.id.iter().map(ToString::to_string).collect();
        let type_str: Vec<&str> = self.r#type.iter().map(ThingType::as_str).collect();
        format!("{}{}?id={}&type={}&versions={}&videos={}&stats={}&historical={}&marketplace={}&ratingcomments={}&page={}&pagesize={}",
            BASEURL,
            self.action,
            id_str.join(","),
            type_str.join(","),
            self.versions as usize,
            self.videos as usize,
            self.stats as usize,
            self.historical as usize,
            self.marketplace as usize,
            self.ratingcomments as usize,
            self.page,
            self.pagesize
        )
    }
}

pub struct Collection {
    action: String,
    username: String,
    version: bool,
    subtype: Vec<ThingType>,
    excludessubtype: Vec<ThingType>,
    id: Vec<usize>,
    brief: bool,
    stats: bool,
    bool_args: HashMap<CollectionArgument, bool>,
    wishlistpriority: Option<usize>,
    minrating: Option<usize>,
    rating: Option<usize>,
    minbggrating: Option<usize>,
    minplays: Option<usize>,
    maxplays: Option<usize>,
    showprivate: Option<usize>,
    collid: Option<usize>,
    modifiedsince: Option<Date<Utc>>
}

impl Collection {
    pub fn new(username: String) -> Collection {
        Collection {
            action: "collection".to_string(),
            username: username,
            version: true,
            subtype: vec![ThingType::Boardgame],
            excludessubtype: Vec::new(),
            id: Vec::new(),
            brief: true,
            stats: true,
            bool_args: HashMap::new(),
            wishlistpriority: None,
            minrating: None,
            rating: None,
            minbggrating: None,
            minplays: None,
            maxplays: None,
            showprivate: None,
            collid: None,
            modifiedsince: None,
        }
    }

    pub fn modified_since<'a>(&'a mut self, date: Date<Utc>) -> &'a mut Collection {
        self.modifiedsince = Some(date);
        self
    }

    pub fn with_subtype<'a>(&'a mut self, subtype: ThingType) -> &'a mut Collection {
        self.subtype.push(subtype);
        self.subtype.sort();
        self.subtype.dedup();
        self
    }

    pub fn set_filter<'a>(&'a mut self, arg: CollectionArgument, val: bool) -> &'a mut Collection {
        self.bool_args.insert(arg, val);
        self
    }


    pub fn unset_filter<'a>(&'a mut self, arg: CollectionArgument) -> &'a mut Collection {
        self.bool_args.remove(&arg);
        self
    }


    pub fn to_string<'a>(&'a mut self) -> String {
        let subtype_str: Vec<&str> = self.subtype.iter().map(ThingType::as_str).collect();
        let mut s: String = format!("{}{}?username={}&version={}&subtype={}&brief={}&stats={}",
            BASEURL,
            self.action,
            self.username,
            self.version as usize,
            subtype_str.join(","),
            self.brief as usize,
            self.stats as usize
        );
        for (key, val) in &self.bool_args {
            s.push_str(&format!("&{}={}", key.as_str(), *val as usize));
        }
        if self.modifiedsince.is_some() {
            s.push_str(&format!("&modifiedsince={}", self.modifiedsince.unwrap().format("%y-%m-%d").to_string()));
        }
        s
    }
}
