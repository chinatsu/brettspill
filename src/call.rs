use chrono::prelude::*;

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
    own: Option<bool>,
    rated: Option<bool>,
    played: Option<bool>,
    comment: Option<bool>,
    trade: Option<bool>,
    want: Option<bool>,
    wishlist: Option<bool>,
    wishlistpriority: Option<usize>,
    preordered: Option<bool>,
    wanttoplay: Option<bool>,
    wanttobuy: Option<bool>,
    prevowned: Option<bool>,
    hasparts: Option<bool>,
    wantparts: Option<bool>,
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
            own: None,
            rated: None,
            played: None,
            comment: None,
            trade: None,
            want: None,
            wishlist: None,
            wishlistpriority: None,
            preordered: None,
            wanttoplay: None,
            wanttobuy: None,
            prevowned: None,
            hasparts: None,
            wantparts: None,
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
        match arg {
            CollectionArgument::Ownership => { self.own = Some(val); }
            CollectionArgument::Rated => { self.rated = Some(val); }
            CollectionArgument::Played => { self.played = Some(val); }
            CollectionArgument::Commented => { self.comment = Some(val); }
            CollectionArgument::MarkedForTrade => { self.trade = Some(val); }
            CollectionArgument::WantedForTrade => { self.want = Some(val); }
            CollectionArgument::Wishlisted => { self.wishlist = Some(val); }
            CollectionArgument::Preordered => { self.preordered = Some(val); }
            CollectionArgument::WantToPlay => { self.wanttoplay = Some(val); }
            CollectionArgument::WantToBuy => { self.wanttobuy = Some(val); }
            CollectionArgument::PreviouslyOwned => { self.prevowned = Some(val); }
            CollectionArgument::HasParts => { self.hasparts = Some(val); }
            CollectionArgument::WantParts => { self.wantparts = Some(val); }
        }
        self
    }


    pub fn unset_filter<'a>(&'a mut self, arg: CollectionArgument) -> &'a mut Collection {
        match arg {
            CollectionArgument::Ownership => { self.own = None; }
            CollectionArgument::Rated => { self.rated = None; }
            CollectionArgument::Played => { self.played = None; }
            CollectionArgument::Commented => { self.comment = None; }
            CollectionArgument::MarkedForTrade => { self.trade = None; }
            CollectionArgument::WantedForTrade => { self.want = None; }
            CollectionArgument::Wishlisted => { self.wishlist = None; }
            CollectionArgument::Preordered => { self.preordered = None; }
            CollectionArgument::WantToPlay => { self.wanttoplay = None; }
            CollectionArgument::WantToBuy => { self.wanttobuy = None; }
            CollectionArgument::PreviouslyOwned => { self.prevowned = None; }
            CollectionArgument::HasParts => { self.hasparts = None; }
            CollectionArgument::WantParts => { self.wantparts = None; }
        }
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
        if self.own.is_some() {
            s.push_str(&format!("&own={}", self.own.unwrap() as usize));
        }
        if self.rated.is_some() {
            s.push_str(&format!("&rated={}", self.rated.unwrap() as usize));
        }
        if self.played.is_some() {
            s.push_str(&format!("&played={}", self.played.unwrap() as usize));
        }
        if self.comment.is_some() {
            s.push_str(&format!("&comment={}", self.comment.unwrap() as usize));
        }
        if self.trade.is_some() {
            s.push_str(&format!("&trade={}", self.trade.unwrap() as usize));
        }
        if self.want.is_some() {
            s.push_str(&format!("&want={}", self.want.unwrap() as usize));
        }
        if self.wishlist.is_some() {
            s.push_str(&format!("&wishlist={}", self.wishlist.unwrap() as usize));
        }
        if self.preordered.is_some() {
            s.push_str(&format!("&preordered={}", self.preordered.unwrap() as usize));
        }
        if self.wanttoplay.is_some() {
            s.push_str(&format!("&wanttoplay={}", self.wanttoplay.unwrap() as usize));
        }
        if self.wanttobuy.is_some() {
            s.push_str(&format!("&wanttobuy={}", self.wanttobuy.unwrap() as usize));
        }
        if self.prevowned.is_some() {
            s.push_str(&format!("&prevowned={}", self.prevowned.unwrap() as usize));
        }
        if self.hasparts.is_some() {
            s.push_str(&format!("&hasparts={}", self.hasparts.unwrap() as usize));
        }
        if self.wantparts.is_some() {
            s.push_str(&format!("&wantparts={}", self.wantparts.unwrap() as usize));
        }
        if self.modifiedsince.is_some() {
            s.push_str(&format!("&modifiedsince={}", self.modifiedsince.unwrap().format("%y-%m-%d").to_string()));
        }
        s
    }
}
