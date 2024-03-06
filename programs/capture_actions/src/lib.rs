use anchor_lang::prelude::*;
// use score_user_actions::cpi::accounts::ScoreUser;
// use score_user_actions::program::ScoreUserActions;
// use score_user_actions::{self, Action};





declare_id!("6LFh4md9KLNGPobngkop6i4nxUuA2C6KLkh8dYo5aUWH");

#[program]
pub mod capture_actions {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }


    pub fn create_post(ctx: Context<CreatePost>, post_id: String) -> Result<()> {

        let post_account = &mut ctx.accounts.post_account;
        let user = *ctx.accounts.user.as_ref().key;


        //create a new post
        let post = Post {
            id: post_id,
            creator: user.clone(),
            downloads: vec![],
            likes: vec![],
            shares: vec![],
            comments: vec![],
            multiple_views: vec![],
            complete_views: vec![],
            timestamp: Clock::get().unwrap().unix_timestamp,
        };

        post_account.posts.push(post);

        //then we score the user
        //first we get the score account
        let score_account = &mut ctx.accounts.score_account;

        //then we check if the user exists on the score account
        score_account.update_user_score(user, Action::Post);

        Ok(())
    }


    pub fn download_post(ctx: Context<DownloadPost>, post_id: String) -> Result<()> {
        //we get the post account
        let post_account = &mut *ctx.accounts.post_account;
        //then we get the index of the post using the id
        let post_index = post_account.clone().find_post_index(post_id);

        //then we check if the post index exists;
        if post_index.is_none() {
            //if it does not we return an error
            return Err(Errors::InvalidPostId.into());
        }else {
            //if it does, we unwrap it;
            let index = post_index.unwrap();
            let user = *ctx.accounts.user.as_ref().key;
            let post = post_account.posts[index].clone();


            //check if the user has downloaded before 
            if post.has_downloaded(user) {
                return Err(Errors::UserAlreadyDownloadedMedia.into())
            }

            //then we get the post and push the user to the downloads
            post_account.posts[index].downloads.push(user);

            //then we score the user
            //first we get the score account
            let score_account = &mut ctx.accounts.score_account;

            //then we check if the user exists on the score account
            score_account.update_user_score(user, Action::Download);
        }

        Ok(())
    }


    pub fn like_post(ctx: Context<LikePost>, post_id: String) -> Result<()> {
        //we get the post account
        let post_account = &mut *ctx.accounts.post_account;
        //then we get the index of the post using the id
        let post_index = post_account.clone().find_post_index(post_id);

        //then we check if the post index exists;
        if post_index.is_none() {
            //if it does not we return an error
            return Err(Errors::InvalidPostId.into());
        }else {
            //if it does, we unwrap it;
            let index = post_index.unwrap();
            let user = *ctx.accounts.user.as_ref().key;
            let post = post_account.posts[index].clone();


            //check if the user has downloaded before 
            if post.has_liked(user) {
                return Err(Errors::UserAlreadyLikedMedia.into())
            }

            //then we get the post and push the user to the downloads
            post_account.posts[index].likes.push(user);

            //then we score the user
            //first we get the score account
            let score_account = &mut ctx.accounts.score_account;

            //then we check if the user exists on the score account
            score_account.update_user_score(user, Action::Like);
        }

        Ok(())
    }


    pub fn comment_post(ctx: Context<CommentPost>, post_id: String) -> Result<()> {
        //we get the post account
        let post_account = &mut *ctx.accounts.post_account;
        //then we get the index of the post using the id
        let post_index = post_account.clone().find_post_index(post_id);

        //then we check if the post index exists;
        if post_index.is_none() {
            //if it does not we return an error
            return Err(Errors::InvalidPostId.into());
        }else {
            //if it does, we unwrap it;
            let index = post_index.unwrap();
            let user = *ctx.accounts.user.as_ref().key;
            let post = post_account.posts[index].clone();


            //check if the user has downloaded before 
            if post.has_commented(user) {
                return Err(Errors::UserAlreadyCommentedOnMedia.into())
            }

            //then we get the post and push the user to the downloads
            post_account.posts[index].comments.push(user);

            //then we score the user
            //first we get the score account
            let score_account = &mut ctx.accounts.score_account;

            //then we check if the user exists on the score account
            score_account.update_user_score(user, Action::Comment);
        }

        Ok(())
    }


    pub fn share_post(ctx: Context<SharePost>, post_id: String) -> Result<()> {
        //we get the post account
        let post_account = &mut *ctx.accounts.post_account;
        //then we get the index of the post using the id
        let post_index = post_account.clone().find_post_index(post_id);

        //then we check if the post index exists;
        if post_index.is_none() {
            //if it does not we return an error
            return Err(Errors::InvalidPostId.into());
        }else {
            //if it does, we unwrap it;
            let index = post_index.unwrap();
            let user = *ctx.accounts.user.as_ref().key;
            let post = post_account.posts[index].clone();


            //check if the user has downloaded before 
            if post.has_shared(user) {
                return Err(Errors::UserAlreadySharedMedia.into())
            }

            //then we get the post and push the user to the downloads
            post_account.posts[index].shares.push(user);

            //then we score the user
            //first we get the score account
            let score_account = &mut ctx.accounts.score_account;

            //then we check if the user exists on the score account
            score_account.update_user_score(user, Action::Share);
        }

        Ok(())
    }



    pub fn complete_post_view(ctx: Context<CompletePostView>, post_id: String) -> Result<()> {
        //first we get the post account
        let post_account = &mut *ctx.accounts.post_account;
        //then we get the post index
        let post_index = post_account.clone().find_post_index(post_id);

        if post_index.is_none() {
            //if it doesn't exist
            return Err(Errors::InvalidPostId.into());
        }else {
            //at this point the post exists
            let user = *ctx.accounts.user.as_ref().key;
            let index = post_index.unwrap();
            let post = post_account.posts[index].clone();

            //then we check if the user has already commented on the post
            if (post.clone()).has_completely_viewed(user){
                //at this point the user has already commented
                //so we check if the user has already viewed multiple times
                if post.has_viewed_multiple_times(user){
                    return Err(Errors::UserAlreadyViewedMediaMultipleTimes.into());
                }else {
                    //at this point the user hasn't viewed multiple times 
                    //so we push the user into the array of multiple views
                    post_account.posts[index].multiple_views.push(user);
                    //then we score the user
                    //first we get the score account
                    let score_account = &mut ctx.accounts.score_account;

                    //then we check if the user exists on the score account
                    score_account.update_user_score(user, Action::MultipleView);
                }
            }else {
                //at this point the user hasn't commented on the post
                //so we add them to the comments array for this post
                post_account.posts[index].complete_views.push(user);

                //then we score the user
                //first we get the score account
                let score_account = &mut ctx.accounts.score_account;

                //then we check if the user exists on the score account
                score_account.update_user_score(user, Action::CompleteView);
            }
        }

        Ok(())
    }



    pub fn reset_user_score(ctx: Context<ClearScore>) -> Result<()> {
        let score_account = &mut ctx.accounts.score_account;
        score_account.clear_user_score(*ctx.accounts.user.as_ref().key);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer=user, space= 10240)]
    pub post_account: Account<'info, PostAccount>,
    #[account(init, payer=user, space= 10240)]
    pub score_account: Account<'info, ScoreAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct DownloadPost<'info> {
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}


#[derive(Accounts)]
pub struct LikePost<'info>{
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}


#[derive(Accounts)]
pub struct CommentPost<'info>{
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}


#[derive(Accounts)]
pub struct SharePost<'info>{
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}


#[derive(Accounts)]
pub struct CompletePostView<'info>{
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct ClearScore<'info> {
    #[account(mut)]
    pub score_account: Account<'info, ScoreAccount>,
    pub user: Signer<'info>
}




#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct Post {
    pub id: String,
    pub creator: Pubkey,
    pub downloads: Vec<Pubkey>,
    pub likes: Vec<Pubkey>,
    pub shares: Vec<Pubkey>,
    pub comments: Vec<Pubkey>,
    pub multiple_views: Vec<Pubkey>,
    pub complete_views: Vec<Pubkey>,
    pub timestamp: i64
}

impl Post {
    fn has_downloaded(self, user: Pubkey) -> bool {
        if self.downloads.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }

    fn has_liked(self, user: Pubkey) -> bool {
        if self.likes.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }
    
    fn has_shared(self, user: Pubkey) -> bool {
        if self.shares.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }

    fn has_commented(self, user: Pubkey) -> bool {
        if self.comments.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }

    fn has_completely_viewed(self, user: Pubkey) -> bool {
        if self.complete_views.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }
    fn has_viewed_multiple_times(self, user: Pubkey) -> bool {
        if self.multiple_views.iter().any(|&d| d == user) {
            true
        }else {
            false
        }
    }

}


#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UserScore {
    pub id: Pubkey,
    pub score: u64,
    pub last_action_timestamp: i64
}




impl UserScore {
    fn new(user_id: Pubkey) -> UserScore {
        UserScore {
            id: user_id,
            score: 0,
            last_action_timestamp: Clock::get().unwrap().unix_timestamp
        }
    }
}


//actions enum
#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Action {
    Download,
    Share,
    Like,
    Comment,
    CompleteView,
    MultipleView,
    Post,
}


#[account]
#[derive(Default)]
pub struct PostAccount {
    pub posts: Vec<Post>
}

impl PostAccount {
    fn find_post_index(self, post_id: String) -> Option<usize> {
        //this checks if any post exists with the post_id
        if self.posts.iter().any(|p| p.id == post_id){
            //if it does we return it's position
            self.posts.iter().position(|p| p.id == post_id)
        }else {
            //if it doesnt exists we return None
            None
        }
    }
}


#[account]
#[derive(Default)]
pub struct ScoreAccount {
    pub scores: Vec<UserScore>
}


impl ScoreAccount {
    fn update_user_score(&mut self, user: Pubkey, action: Action) {
        let score = match action {
            Action::Download => 2,
            Action::Comment => 4,
            Action::Post => 10,
            Action::Like => 1,
            Action::Share => 2,
            Action::CompleteView => 2,
            Action::MultipleView => 1,
        };
        if self.scores.iter().any(|s| s.id == user){
            //at this point a user exists
            let i = self.scores.iter().position(|s| s.id == user).unwrap();
            self.scores[i].score += score;
            self.scores[i].last_action_timestamp = Clock::get().unwrap().unix_timestamp;
        }else {
            let mut new_user_score = UserScore::new(user);
            //then we update user score
            new_user_score.score += score;
            //then we push the user in the score_account.scores array
            self.scores.push(new_user_score);
        }
    }


    fn clear_user_score(&mut self, user_id: Pubkey){
        //first check if user is the user exists
        let user_index = self.scores.iter().position(|s| s.id == user_id).expect("No user found");
        self.scores[user_index].score = 0;
        self.scores[user_index].last_action_timestamp = Clock::get().unwrap().unix_timestamp; 
    }
}






#[error_code]
pub enum Errors {
    #[msg("No post with this id exists")]
    InvalidPostId,
    #[msg("User has already downloaded this post")]
    UserAlreadyDownloadedMedia,
    #[msg("User has already liked this post")]
    UserAlreadyLikedMedia,
    #[msg("User has already commented on this post")]
    UserAlreadyCommentedOnMedia,
    #[msg("User has already shared this post")]
    UserAlreadySharedMedia,
    #[msg("User has already viewed this post multiple times")]
    UserAlreadyViewedMediaMultipleTimes,
}