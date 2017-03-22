//! Collection of statistics about the execution of the algorithm.

/// Collection of statistics about the execution of the algorithm.
///
/// Times are given in nanoseconds.
pub struct Statistics {
    /// Number of friendships in the social graph.
    number_of_friendships: u64,

    /// Number of retweets processed.
    number_of_retweets: u64,

    /// Size of the Retweet batches.
    batch_size: usize,

    /// Time to set up the computation.
    time_to_setup: u64,

    /// Time to load and process the social graph.
    time_to_process_social_graph: u64,

    /// Time to load the retweets.
    time_to_load_retweets: u64,

    /// Time to process the retweets.
    time_to_process_retweets: u64,

    /// Total time of the computation.
    total_time: u64
}

impl Statistics {
    /// Collect statistics about the influence computation. Times must be given in nanoseconds.
    pub fn new(number_of_friendships: u64, number_of_retweets: u64, batch_size: usize, time_to_setup: u64,
               time_to_process_social_graph: u64, time_to_load_retweets: u64, time_to_process_retweets: u64,
               total_time: u64) -> Statistics {
        Statistics { number_of_friendships: number_of_friendships, number_of_retweets: number_of_retweets,
            batch_size: batch_size, time_to_setup: time_to_setup,
            time_to_process_social_graph: time_to_process_social_graph, time_to_load_retweets: time_to_load_retweets,
            time_to_process_retweets: time_to_process_retweets, total_time: total_time }
    }

    /// Get the number of friendships in the social graph.
    pub fn number_of_friendships(&self) -> u64 {
        self.number_of_friendships
    }

    /// Get the total number of retweets processed.
    pub fn number_of_retweets(&self) -> u64 {
        self.number_of_retweets
    }

    /// Get the size of the Retweet batches.
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    /// Get the time to set up the computation (in nanoseconds).
    pub fn time_to_setup(&self) -> u64 {
        self.time_to_setup
    }

    /// Get the time to load and process the social graph (in nanoseconds).
    pub fn time_to_process_social_graph(&self) -> u64 {
        self.time_to_process_social_graph
    }

    /// Get the time to load the retweets (in nanoseconds).
    pub fn time_to_load_retweets(&self) -> u64 {
        self.time_to_load_retweets
    }

    /// Get the time to process the retweets (in nanoseconds).
    pub fn time_to_process_retweets(&self) -> u64 {
        self.time_to_process_retweets
    }

    /// Get the total time it took the computation to finish (in nanoseconds).
    pub fn total_time(&self) -> u64 {
        self.total_time
    }

    /// Get the average Retweet processing rate in Retweets per seconds (RT/s).
    pub fn retweet_processing_rate(&self) -> u64 {
        (self.number_of_retweets as f64 / (self.time_to_process_retweets as f64 / 1_000_000_000.0f64)) as u64
    }
}
