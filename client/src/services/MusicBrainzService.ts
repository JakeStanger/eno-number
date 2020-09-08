import IArtist from "../types/IArtist";
import { throttle } from "lodash";

/**
 * Service class for interacting with
 * external MusicBrainz API
 */
class MusicBrainzService {
  private static instance: MusicBrainzService;

  private artistSearchCache: Record<string, IArtist[]> = {};

  private constructor() {
    MusicBrainzService.instance = this;
    MusicBrainzService.fetch = throttle(MusicBrainzService.fetch, 250, {trailing: false}) as any;
  }

  public static get() {
    return MusicBrainzService.instance || new MusicBrainzService();
  }

  public async searchArtists(query: string): Promise<IArtist[]> {
    if (query) {
      if(this.artistSearchCache[query]) {
        return Promise.resolve(this.artistSearchCache[query]);
      }

      const artists: IArtist[] = await (
        MusicBrainzService.fetch<{ artists: IArtist[] }>(
          `/artist?limit=10&format=json&query=artist:${query}`
        )?.then((r) => r?.artists) || [] as IArtist[]
      );

      this.artistSearchCache[query] = artists;
      return artists;
    }

    return Promise.reject("Must include query");
  }

  private static fetch<T>(url: string): Promise<T> | undefined {
    return fetch(process.env.REACT_APP_MUSICBRAINZ_API + url, {
      headers: {
        Accept: "application/json",
        "Content-Type": "application/json",
      },
    }).then((r) => r.json());
  }
}

export default MusicBrainzService;
