/**
 * Authentication Helper Functions
 * Handles JWT token management, user state, and protected routes
 */

// Get stored access token
function getAccessToken() {
  return localStorage.getItem('accessToken');
}

// Get stored refresh token
function getRefreshToken() {
  return localStorage.getItem('refreshToken');
}

// Get stored user
function getUser() {
  const userJson = localStorage.getItem('user');
  return userJson ? JSON.parse(userJson) : null;
}

// Check if user is authenticated
function isAuthenticated() {
  return !!getAccessToken();
}

// Logout user
function logout() {
  localStorage.removeItem('accessToken');
  localStorage.removeItem('refreshToken');
  localStorage.removeItem('user');
}

// Refresh access token
async function refreshAccessToken() {
  const refreshToken = getRefreshToken();
  if (!refreshToken) {
    throw new Error('No refresh token');
  }

  const response = await fetch('/api/auth/refresh', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ refreshToken })
  });

  if (!response.ok) {
    throw new Error('Token refresh failed');
  }

  const data = await response.json();
  localStorage.setItem('accessToken', data.accessToken);
  return data.accessToken;
}

// Make authenticated fetch request
async function authenticatedFetch(url, options = {}) {
  let token = getAccessToken();

  if (!token) {
    throw new Error('Not authenticated');
  }

  // Add authorization header
  options.headers = {
    ...options.headers,
    'Authorization': `Bearer ${token}`
  };

  let response = await fetch(url, options);

  // If unauthorized, try to refresh token
  if (response.status === 401 && getRefreshToken()) {
    try {
      token = await refreshAccessToken();

      // Retry with new token
      options.headers['Authorization'] = `Bearer ${token}`;
      response = await fetch(url, options);
    } catch (err) {
      // Refresh failed, logout
      logout();
      window.location.href = '/login.html';
      throw err;
    }
  }

  return response;
}

// Require authentication (redirect to login if not authenticated)
function requireAuth() {
  if (!isAuthenticated()) {
    window.location.href = '/login.html';
    return false;
  }
  return true;
}

// Redirect if already authenticated (for login/register pages)
function redirectIfAuthenticated() {
  if (isAuthenticated()) {
    window.location.href = '/dashboard.html';
    return true;
  }
  return false;
}

// Show error message
function showError(message) {
  // Create or update error toast
  let toast = document.getElementById('error-toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.id = 'error-toast';
    toast.className = 'toast error-toast';
    document.body.appendChild(toast);
  }

  toast.textContent = message;
  toast.classList.add('show');

  setTimeout(() => {
    toast.classList.remove('show');
  }, 5000);
}

// Show success message
function showSuccess(message) {
  let toast = document.getElementById('success-toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.id = 'success-toast';
    toast.className = 'toast success-toast';
    document.body.appendChild(toast);
  }

  toast.textContent = message;
  toast.classList.add('show');

  setTimeout(() => {
    toast.classList.remove('show');
  }, 5000);
}

// Check and enforce tier limits
function checkTierLimit(resource, current, user = null) {
  user = user || getUser();
  if (!user) return false;

  const limits = {
    free: { nodes: 5 },
    pro: { nodes: 20 },
    enterprise: { nodes: -1 }
  };

  const limit = limits[user.tier][resource];
  if (limit === -1) return true; // Unlimited

  return current < limit;
}

// Format date/time
function formatDateTime(timestamp) {
  const date = new Date(timestamp);
  return date.toLocaleString();
}

// Format relative time
function formatRelativeTime(timestamp) {
  const now = Date.now();
  const diff = now - timestamp;

  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) return `${days} day${days > 1 ? 's' : ''} ago`;
  if (hours > 0) return `${hours} hour${hours > 1 ? 's' : ''} ago`;
  if (minutes > 0) return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
  return 'Just now';
}

// Export for use in other scripts
if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    getAccessToken,
    getRefreshToken,
    getUser,
    isAuthenticated,
    logout,
    refreshAccessToken,
    authenticatedFetch,
    requireAuth,
    redirectIfAuthenticated,
    showError,
    showSuccess,
    checkTierLimit,
    formatDateTime,
    formatRelativeTime
  };
}
