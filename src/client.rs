use crate::query::{
    DivisionMatchesQuery, DivisionRankingsQuery, EventAwardsQuery, EventSkillsQuery,
    EventTeamsQuery, SeasonEventsQuery, TeamAwardsQuery, TeamEventsQuery, TeamMatchesQuery,
    TeamRankingsQuery, TeamSkillsQuery,
};

use super::{
    query::{EventsQuery, SeasonsQuery, TeamsQuery},
    schema::*,
};
use std::time::Duration;

#[derive(Default, Debug, Clone)]
pub struct RobotEvents {
    pub bearer_token: String,
    pub req_client: reqwest::Client,
}

pub const V1_API_BASE: &str = "https://www.robotevents.com/api";
pub const V2_API_BASE: &str = "https://www.robotevents.com/api/v2";

impl RobotEvents {
    /// Creates a new RobotEvents API client.
    ///
    /// A bearer authentication token is required for requests to be made. This can
    /// be obtained from RobotEvents by creating an account and requesting one.
    /// 
    /// # Examples
    /// 
    /// Creating a client with a token stored as an enviornment variable:
    /// 
    /// ```
    /// use robotevents::RobotEvents;
    /// 
    /// let token = std::env::var("ROBOTEVENTS_TOKEN")?;
    /// let client = RobotEvents::new(token);
    /// ```
    pub fn new(bearer_token: impl AsRef<str>) -> Self {
        Self {
            bearer_token: bearer_token.as_ref().to_owned(),
            req_client: reqwest::Client::new(),
        }
    }

    /// Make a request to a [RobotEvents API v2](https://www.robotevents.com/api/v2) endpoint using the
    /// client's bearer token.
    pub async fn request(
        &self,
        endpoint: impl AsRef<str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .req_client
            .get(format!("{V2_API_BASE}{}", endpoint.as_ref()))
            .bearer_auth(&self.bearer_token)
            .timeout(Duration::from_secs(10))
            .send()
            .await?)
    }

    /// Make a request to a RobotEvents API v1 endpoint.
    pub async fn request_api_v1(
        &self,
        endpoint: impl AsRef<str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .req_client
            .get(format!("{V1_API_BASE}{}", endpoint.as_ref()))
            .timeout(Duration::from_secs(10))
            .send()
            .await?)
    }

    /////////////////////////////////////////////////////////////////////////
    // Team-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of [`Team`]s from RobotEvents.
    ///
    /// Team listings can be queryed using a [`TeamsQuery`] search.
    pub async fn teams(
        &self,
        query: TeamsQuery,
    ) -> Result<PaginatedResponse<Team>, reqwest::Error> {
        Ok(self
            .request(format!("/teams{query}"))
            .await?
            .json()
            .await?)
    }

    /// Get a specific RobotEvents [`Team`] by ID.
    pub async fn team(&self, team_id: i32) -> Result<Team, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Event`]s that a given Team ID has attended.
    pub async fn team_events(
        &self,
        team_id: i32,
        query: TeamEventsQuery,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/events{query}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Match`]es that a given Team ID has played in.
    pub async fn team_matches(
        &self,
        team_id: i32,
        query: TeamMatchesQuery,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/matches{query}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Ranking`]s that a given Team ID has played in.
    pub async fn team_rankings(
        &self,
        team_id: i32,
        query: TeamRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/rankings{query}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Skill`]s runs that a given Team ID has performed.
    pub async fn team_skills(
        &self,
        team_id: i32,
        query: TeamSkillsQuery,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/skills{query}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of [`Award`]s that a given Team ID has received.
    pub async fn team_awards(
        &self,
        team_id: i32,
        query: TeamAwardsQuery,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        Ok(self
            .request(format!("/teams/{team_id}/awards{query}"))
            .await?
            .json()
            .await?)
    }

    
    /////////////////////////////////////////////////////////////////////////
    // Season-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of [`Season`]s from RobotEvents.
    ///
    /// Season listings can be queryed using a [`SeasonQuery`] search.
    pub async fn seasons(
        &self,
        query: SeasonsQuery,
    ) -> Result<PaginatedResponse<Season>, reqwest::Error> {
        Ok(self
            .request(format!("/seasons{query}"))
            .await?
            .json()
            .await?)
    }

    /// Get a specific RobotEvents [`Season`] by ID.
    pub async fn season(&self, season_id: i32) -> Result<Season, reqwest::Error> {
        Ok(self
            .request(format!("/seasons/{season_id}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Events for a given Season.
    pub async fn season_events(
        &self,
        season_id: i32,
        query: SeasonEventsQuery,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/seasons/{season_id}/events{query}"))
            .await?
            .json()
            .await?)
    }

    
    /////////////////////////////////////////////////////////////////////////
    // Program-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of all programs from RobotEvents.
    pub async fn programs(&self) -> Result<PaginatedResponse<IdInfo>, reqwest::Error> {
        Ok(self.request("/programs").await?.json().await?)
    }
    /// Get a specific RobotEvents program by ID.
    pub async fn program(&self, program_id: i32) -> Result<IdInfo, reqwest::Error> {
        Ok(self
            .request(format!("/programs/{program_id}"))
            .await?
            .json()
            .await?)
    }

    
    /////////////////////////////////////////////////////////////////////////
    // Event-related endpoint methods
    /////////////////////////////////////////////////////////////////////////

    /// Get a paginated list of [`Event`]s from RobotEvents.
    ///
    /// Event listings can be queryed using an [`EventQuery`] search.
    pub async fn events(
        &self,
        query: EventsQuery,
    ) -> Result<PaginatedResponse<Event>, reqwest::Error> {
        Ok(self
            .request(format!("/events{query}"))
            .await?
            .json()
            .await?)
    }

    /// Get a specific RobotEvents event by ID.
    pub async fn event(&self, event_id: i32) -> Result<Event, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of teams attending an event.
    pub async fn event_teams(
        &self,
        event_id: i32,
        query: EventTeamsQuery,
    ) -> Result<PaginatedResponse<Team>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/teams{query}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of skills runs at an event.
    pub async fn event_skills(
        &self,
        event_id: i32,
        query: EventSkillsQuery,
    ) -> Result<PaginatedResponse<Skill>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/skills{query}"))
            .await?
            .json()
            .await?)
    }

    /// Get a paginated list of skills runs at an event.
    pub async fn event_awards(
        &self,
        event_id: i32,
        query: EventAwardsQuery,
    ) -> Result<PaginatedResponse<Award>, reqwest::Error> {
        Ok(self
            .request(format!("/events/{event_id}/awards{query}"))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Matches for a single Division of an Event.
    pub async fn event_division_matches(
        &self,
        event_id: i32,
        division_id: i32,
        query: DivisionMatchesQuery,
    ) -> Result<PaginatedResponse<Match>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/matches{query}"
            ))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Finalist Rankings for a single Division of an Event.
    pub async fn event_division_finalist_rankings(
        &self,
        event_id: i32,
        division_id: i32,
        query: DivisionRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/finalistRankings{query}"
            ))
            .await?
            .json()
            .await?)
    }

    /// Gets a List of Rankings for a single Division of an Event.
    pub async fn event_division_rankings(
        &self,
        event_id: i32,
        division_id: i32,
        query: DivisionRankingsQuery,
    ) -> Result<PaginatedResponse<Ranking>, reqwest::Error> {
        Ok(self
            .request(format!(
                "/events/{event_id}/divisions/{division_id}/finalist{query}"
            ))
            .await?
            .json()
            .await?)
    }
}
