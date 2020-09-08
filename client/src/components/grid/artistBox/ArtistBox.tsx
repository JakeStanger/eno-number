import React, { forwardRef, useMemo } from 'react';
import IArtistBoxProps from "./IArtistBoxProps";
import styles from "./ArtistBox.module.scss";
import DatabaseService from "../../../services/DatabaseService";
import AsyncImage from "./asyncImage/AsyncImage";
import Draggable from "react-draggable";

const ArtistBox: React.FC<IArtistBoxProps> = forwardRef(
  ({ artist, x, y, onDragStop }, ref: any) => {

    const artSource = useMemo(() => DatabaseService.get().getArtistArt(artist), [artist])

    return (
      <Draggable
        defaultPosition={{ x, y }}
        onDrag={onDragStop}
        handle={".drag-handle"}
      >
        <div ref={ref} className={styles.artistBox}>
          <div className={`drag-handle ${styles.dragHandle}`}>
            <div>{artist.name}</div>
          </div>
          <AsyncImage src={artSource} width={150} height={150} />
        </div>
      </Draggable>
    );
  }
);

export default ArtistBox;
