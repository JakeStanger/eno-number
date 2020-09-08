import React, { useEffect, useState } from 'react';
import IAsyncImageProps from './IAsyncImageProps';

const AsyncImage: React.FC<IAsyncImageProps> = (props) => {
  const { src } = props;

  const [source, setSource] = useState<string | null>(null);

  useEffect(() => {
    src.then(setSource);
  }, [src]);

  return <img {...props as any} src={source} alt={props.alt} />;
};

export default AsyncImage;
