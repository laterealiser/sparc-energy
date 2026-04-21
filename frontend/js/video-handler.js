/**
 * Sparc Energy - Video Handler
 * Handles video loading, error states, and fallbacks
 */

class VideoHandler {
    constructor() {
        this.videos = document.querySelectorAll('video.vid-bg');
        this.init();
    }

    init() {
        this.videos.forEach(video => {
            this.setupVideo(video);
        });
    }

    setupVideo(video) {
        // Add loading state
        video.addEventListener('loadstart', () => {
            console.log('Video loading started:', video.currentSrc);
            this.showLoadingState(video);
        });

        // Handle successful load
        video.addEventListener('canplay', () => {
            console.log('Video can play:', video.currentSrc);
            this.hideLoadingState(video);
        });

        // Handle errors
        video.addEventListener('error', (e) => {
            console.error('Video error:', e, video.currentSrc);
            this.handleVideoError(video);
        });

        // Handle stalled loading
        video.addEventListener('stalled', () => {
            console.warn('Video stalled:', video.currentSrc);
            setTimeout(() => {
                if (video.readyState < 3) {
                    this.handleVideoError(video);
                }
            }, 10000); // 10 second timeout
        });

        // Ensure autoplay works
        video.addEventListener('loadedmetadata', () => {
            const playPromise = video.play();
            if (playPromise !== undefined) {
                playPromise.catch(error => {
                    console.warn('Autoplay failed:', error);
                    // Autoplay failed, but that's okay for background videos
                    // User interaction will be required to play
                });
            }
        });

        // Handle network errors
        video.addEventListener('abort', () => {
            console.warn('Video loading aborted:', video.currentSrc);
        });

        // Monitor loading progress
        video.addEventListener('progress', () => {
            if (video.buffered.length > 0) {
                const bufferedEnd = video.buffered.end(video.buffered.length - 1);
                const duration = video.duration;
                if (duration > 0) {
                    const percent = (bufferedEnd / duration) * 100;
                    console.log(`Video buffered: ${percent.toFixed(1)}%`);
                }
            }
        });
    }

    showLoadingState(video) {
        const container = video.closest('.vid-bg-wrap');
        if (container) {
            container.classList.add('loading');
        }
    }

    hideLoadingState(video) {
        const container = video.closest('.vid-bg-wrap');
        if (container) {
            container.classList.remove('loading');
        }
    }

    handleVideoError(video) {
        console.error('Video failed to load, showing fallback');
        
        // Hide the video
        video.style.display = 'none';
        
        // Show fallback background
        const container = video.closest('.vid-bg-wrap');
        if (container) {
            container.classList.add('video-error');
            
            // Create fallback if it doesn't exist
            let fallback = container.querySelector('.video-fallback');
            if (!fallback) {
                fallback = document.createElement('div');
                fallback.className = 'video-fallback';
                fallback.style.cssText = `
                    position: absolute;
                    top: 0;
                    left: 0;
                    width: 100%;
                    height: 100%;
                    background: linear-gradient(135deg, #0a1f12 0%, #1a4a2e 100%);
                    z-index: 1;
                `;
                container.appendChild(fallback);
            }
            fallback.style.display = 'block';
        }
    }

    // Method to check if videos are loading properly
    checkVideoHealth() {
        const report = {
            total: this.videos.length,
            loaded: 0,
            errors: 0,
            loading: 0
        };

        this.videos.forEach(video => {
            if (video.readyState >= 3) {
                report.loaded++;
            } else if (video.error) {
                report.errors++;
            } else {
                report.loading++;
            }
        });

        console.log('Video Health Report:', report);
        return report;
    }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.videoHandler = new VideoHandler();
    
    // Check video health after 5 seconds
    setTimeout(() => {
        if (window.videoHandler) {
            window.videoHandler.checkVideoHealth();
        }
    }, 5000);
});

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
    module.exports = VideoHandler;
}