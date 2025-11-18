/**
 * Location Service - Handles GPS and location-based operations
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import * as Location from 'expo-location';
import { Coordinates } from '../types/atm.types';

class LocationService {
  private currentLocation: Coordinates | null = null;

  /**
   * Request location permissions
   */
  async requestLocationPermission(): Promise<boolean> {
    try {
      const { status } = await Location.requestForegroundPermissionsAsync();
      return status === 'granted';
    } catch (error) {
      console.error('Error requesting location permission:', error);
      return false;
    }
  }

  /**
   * Check if location permissions are granted
   */
  async hasLocationPermission(): Promise<boolean> {
    try {
      const { status } = await Location.getForegroundPermissionsAsync();
      return status === 'granted';
    } catch (error) {
      console.error('Error checking location permission:', error);
      return false;
    }
  }

  /**
   * Get current location
   */
  async getCurrentLocation(): Promise<Coordinates> {
    try {
      const hasPermission = await this.hasLocationPermission();

      if (!hasPermission) {
        const granted = await this.requestLocationPermission();
        if (!granted) {
          throw new Error('Location permission denied');
        }
      }

      const location = await Location.getCurrentPositionAsync({
        accuracy: Location.Accuracy.Balanced,
      });

      this.currentLocation = {
        latitude: location.coords.latitude,
        longitude: location.coords.longitude,
      };

      return this.currentLocation;
    } catch (error) {
      console.error('Error getting current location:', error);
      throw error;
    }
  }

  /**
   * Get cached location
   */
  getCachedLocation(): Coordinates | null {
    return this.currentLocation;
  }

  /**
   * Watch location updates
   */
  async watchLocation(
    callback: (location: Coordinates) => void
  ): Promise<Location.LocationSubscription> {
    try {
      const hasPermission = await this.hasLocationPermission();

      if (!hasPermission) {
        const granted = await this.requestLocationPermission();
        if (!granted) {
          throw new Error('Location permission denied');
        }
      }

      const subscription = await Location.watchPositionAsync(
        {
          accuracy: Location.Accuracy.Balanced,
          timeInterval: 10000, // Update every 10 seconds
          distanceInterval: 100, // Update every 100 meters
        },
        (location) => {
          const coords: Coordinates = {
            latitude: location.coords.latitude,
            longitude: location.coords.longitude,
          };

          this.currentLocation = coords;
          callback(coords);
        }
      );

      return subscription;
    } catch (error) {
      console.error('Error watching location:', error);
      throw error;
    }
  }

  /**
   * Calculate distance between two coordinates (Haversine formula)
   */
  calculateDistance(
    lat1: number,
    lng1: number,
    lat2: number,
    lng2: number
  ): number {
    const R = 3959; // Earth's radius in miles
    const dLat = this.toRad(lat2 - lat1);
    const dLng = this.toRad(lng2 - lng1);

    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(this.toRad(lat1)) *
        Math.cos(this.toRad(lat2)) *
        Math.sin(dLng / 2) *
        Math.sin(dLng / 2);

    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    return R * c;
  }

  /**
   * Convert degrees to radians
   */
  private toRad(degrees: number): number {
    return degrees * (Math.PI / 180);
  }

  /**
   * Format distance for display
   */
  formatDistance(miles: number): string {
    if (miles < 0.1) {
      return 'Less than 0.1 mi';
    } else if (miles < 1) {
      return `${miles.toFixed(1)} mi`;
    } else {
      return `${miles.toFixed(1)} mi`;
    }
  }

  /**
   * Get address from coordinates (reverse geocoding)
   */
  async getAddressFromCoordinates(
    latitude: number,
    longitude: number
  ): Promise<string> {
    try {
      const addresses = await Location.reverseGeocodeAsync({
        latitude,
        longitude,
      });

      if (addresses.length > 0) {
        const address = addresses[0];
        const parts = [
          address.street,
          address.city,
          address.region,
          address.postalCode,
        ].filter(Boolean);

        return parts.join(', ');
      }

      return 'Unknown location';
    } catch (error) {
      console.error('Error reverse geocoding:', error);
      return 'Unknown location';
    }
  }

  /**
   * Get coordinates from address (forward geocoding)
   */
  async getCoordinatesFromAddress(address: string): Promise<Coordinates | null> {
    try {
      const locations = await Location.geocodeAsync(address);

      if (locations.length > 0) {
        return {
          latitude: locations[0].latitude,
          longitude: locations[0].longitude,
        };
      }

      return null;
    } catch (error) {
      console.error('Error geocoding:', error);
      return null;
    }
  }

  /**
   * Open navigation app (Google Maps or Apple Maps)
   */
  async openNavigation(
    destination: Coordinates,
    label?: string
  ): Promise<void> {
    try {
      const { latitude, longitude } = destination;
      const label_encoded = label ? encodeURIComponent(label) : 'Destination';

      // Try Google Maps first, fallback to Apple Maps
      const url = `https://www.google.com/maps/dir/?api=1&destination=${latitude},${longitude}&destination_place_id=${label_encoded}`;

      await Location.openMapsAsync({
        latitude,
        longitude,
      });
    } catch (error) {
      console.error('Error opening navigation:', error);
      throw error;
    }
  }

  /**
   * Check if location services are enabled
   */
  async isLocationEnabled(): Promise<boolean> {
    try {
      return await Location.hasServicesEnabledAsync();
    } catch (error) {
      console.error('Error checking location services:', error);
      return false;
    }
  }
}

export default new LocationService();
