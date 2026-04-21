# Video Optimization Guide for Sparc Energy

## 🚨 Current Issues Found

### 1. **Large File Size**
- `outcome.mp4` is **87.93 MB** - too large for web use
- Recommended max size: **10-15 MB** for background videos

### 2. **Missing WebM Format**
- Only MP4 files exist, no WebM alternatives
- WebM provides better compression and browser support

### 3. **No Progressive Loading**
- Videos load entirely before playing
- Should use progressive/streaming loading

## 🔧 Immediate Fixes Applied

### 1. **Error Handling**
- Added fallback backgrounds for failed video loads
- Implemented loading states and error detection
- Added video health monitoring

### 2. **Browser Compatibility**
- Added `preload="auto"` for better loading
- Added multiple source formats (MP4 + WebM)
- Added proper MIME type configuration

### 3. **Server Configuration**
- Created `.htaccess` with proper video MIME types
- Enabled range requests for video seeking
- Added caching headers for better performance

## 📋 Required Actions

### 1. **Optimize Video Files**
```bash
# Compress the large outcome.mp4 file
ffmpeg -i outcome.mp4 -c:v libx264 -crf 28 -preset slow -c:a aac -b:a 128k outcome_optimized.mp4

# Create WebM versions for all videos
ffmpeg -i hero.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus hero.webm
ffmpeg -i crisis.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus crisis.webm
ffmpeg -i method.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus method.webm
ffmpeg -i outcome.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus outcome.webm
ffmpeg -i consultation.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus consultation.webm
ffmpeg -i dashboard.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus dashboard.webm
ffmpeg -i why-sparc.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus why-sparc.webm
```

### 2. **Recommended Video Specs**
- **Resolution**: 1920x1080 (Full HD max)
- **Duration**: 10-30 seconds for loops
- **File Size**: Under 15MB per video
- **Bitrate**: 2-4 Mbps for video, 128kbps for audio
- **Format**: MP4 (H.264) + WebM (VP9) for compatibility

### 3. **Alternative Solutions**

#### Option A: Use Poster Images
Replace heavy videos with high-quality poster images:
```html
<video poster="assets/images/hero-poster.jpg" preload="none">
  <source src="assets/videos/hero.mp4" type="video/mp4">
</video>
```

#### Option B: Lazy Load Videos
Only load videos when they come into viewport:
```javascript
// Implemented in video-handler.js
const observer = new IntersectionObserver((entries) => {
  entries.forEach(entry => {
    if (entry.isIntersecting) {
      const video = entry.target.querySelector('video');
      if (video && !video.src) {
        video.load();
      }
    }
  });
});
```

#### Option C: Use CSS Gradients as Fallbacks
For immediate loading, use CSS backgrounds:
```css
.hero {
  background: linear-gradient(135deg, #0a1f12 0%, #1a4a2e 100%);
}
```

## 🔍 Debugging Video Issues

### Check Video Loading Status
Open browser console and run:
```javascript
window.videoHandler.checkVideoHealth()
```

### Common Issues & Solutions

1. **Videos not playing on mobile**
   - Solution: Added `playsinline` attribute ✅
   - Autoplay may be blocked - this is normal for background videos

2. **Videos not loading**
   - Check network tab for 404 errors
   - Verify MIME types are correct
   - Check file permissions

3. **Slow loading**
   - Compress video files
   - Use CDN for video hosting
   - Implement progressive loading

4. **Browser compatibility**
   - Provide multiple formats (MP4 + WebM) ✅
   - Add fallback backgrounds ✅

## 📊 Current File Sizes
```
consultation.mp4:  9.87 MB ✅ (Good)
crisis.mp4:       14.20 MB ⚠️  (Acceptable)
dashboard.mp4:    10.94 MB ✅ (Good)
hero.mp4:          8.84 MB ✅ (Good)
method.mp4:       18.38 MB ⚠️  (Should compress)
outcome.mp4:      87.93 MB ❌ (Too large - must compress)
why-sparc.mp4:    16.32 MB ⚠️  (Should compress)
```

## 🚀 Performance Recommendations

1. **Immediate**: Compress `outcome.mp4` to under 15MB
2. **Short-term**: Create WebM versions of all videos
3. **Long-term**: Consider using a video CDN like Cloudflare or AWS CloudFront
4. **Alternative**: Use high-quality images with CSS animations instead of videos

## 🔧 Testing Checklist

- [ ] Videos load on Chrome desktop
- [ ] Videos load on Safari desktop  
- [ ] Videos load on Chrome mobile
- [ ] Videos load on Safari mobile
- [ ] Fallbacks work when videos fail
- [ ] Loading states appear correctly
- [ ] No console errors
- [ ] Page loads in under 3 seconds on 3G