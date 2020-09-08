import IArtist from "../types/IArtist";
import ICalculation from "../types/ICalculation";

class DatabaseService {
  private static instance: DatabaseService;

  private artistSearchCache: Record<string, IArtist[]> = {}; // TODO: Cache to localstorage (or indexeddb)

  private constructor() {
    DatabaseService.instance = this;
  }

  public static get() {
    return DatabaseService.instance || new DatabaseService();
  }

  public async searchArtists(query: string): Promise<IArtist[]> {
    query = query.trim();
    if (query) {
      if (this.artistSearchCache[query]) {
        return Promise.resolve(this.artistSearchCache[query]);
      }

      const artists: IArtist[] = await fetch(
        `/api/search/artist/${query}`
      ).then((r) => r.json());

      this.artistSearchCache[query] = artists;
      return artists;
    }

    return Promise.reject("Must include query");
  }

  /**
   * Gets the source URL for the
   * artist's art.
   * @param artist The artist
   * @param full Whether to fetch the full res image
   */
  public async getArtistArt(artist: IArtist, full = false): Promise<string> {
    return fetch(`/api/art/artist/${artist.id}?full=${full.toString()}`).then((r) =>
      r.text()
    );
  }

  public async calculate(
    start: IArtist,
    destination: IArtist
  ): Promise<ICalculation> {
    return fetch("/api/calculate", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        start: start.id,
        destination: destination.id,
      }),
    }).then((r) => r.json());
  }
}

export default DatabaseService;
