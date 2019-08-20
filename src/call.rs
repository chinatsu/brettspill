const BASEURL: &str = "https://www.boardgamegeek.com/xmlapi2/";

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
        self
    }

    pub fn with_type<'a>(&'a mut self, thingtype: ThingType) -> &'a mut Thing {
        self.r#type.push(thingtype);
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
