import IArtist from '../../../types/IArtist';

interface IArtistBoxProps {
  artist: IArtist;
  x: number;
  y: number;
  onDragStop: VoidFunction;
}

export default IArtistBoxProps;
