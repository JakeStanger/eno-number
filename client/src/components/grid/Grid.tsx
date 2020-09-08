import React, { MutableRefObject, useCallback, useEffect, useRef, useState } from 'react';
import IGridProps from "./IGridProps";
import styles from "./Grid.module.scss";
import IArtistBoxProps from "./artistBox/IArtistBoxProps";
import IArtist from "../../types/IArtist";
// import DraggableArtistBox from "./artistBox/DraggableArtistBox.tsx.bak";
// import CustomDragLayer from "./CustomDragLayer.tsx.bak";
import ArtistBox from "./artistBox/ArtistBox";
import { ArcherContainer, ArcherElement, Relation } from "react-archer";
import { random } from 'lodash';

function getRelations(index: number, array: IArtistBoxProps[]): Relation[] {
  const next = array[index + 1];
  if (next !== undefined) {
    return [
      {
        targetId: next.artist.id.toString(),
        targetAnchor: "middle",
        sourceAnchor: "middle",
      },
    ];
  }
  return [];
}

const Grid: React.FC<IGridProps> = ({ artists }) => {
  const [artistBoxes, setArtistBoxes] = useState<IArtistBoxProps[]>([]);

  const arrowRef = useRef<ArcherContainer>();

  const redrawArrows = useCallback(() => {
    arrowRef.current?.refreshScreen();
  }, [arrowRef]);

  // set initial positions
  useEffect(() => {
    setArtistBoxes(
      artists.map((artist, i) => ({
        artist,
        x: 0,
        y: random(10, 30) * 16,
        arrowRef: null,
        onDragStop: redrawArrows
      }))
    );
  }, [redrawArrows, artists]);

  return (
    // <div className={styles.gridContainer}>
    <ArcherContainer
      className={styles.gridContainer}
      svgContainerStyle={{ zIndex: -1 }}
      strokeColor={"#2d2d2d"}
      ref={arrowRef as MutableRefObject<ArcherContainer>}
    >
      <div className={styles.grid}>
        {artistBoxes.map((box, i, arr) => (
          <ArcherElement
            key={box.artist.id}
            id={box.artist.id.toString()}
            relations={getRelations(i, arr)}
          >
            <ArtistBox {...box} />
          </ArcherElement>
        ))}
      </div>
    </ArcherContainer>
    // </div>
  );
};

export default Grid;
