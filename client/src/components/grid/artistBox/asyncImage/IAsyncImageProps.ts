import React from 'react';

interface IAsyncImageProps extends Omit<React.HTMLProps<HTMLImageElement>, "src"> {
  src: Promise<string>;
}

export default IAsyncImageProps;