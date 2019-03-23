class Functor f =
  fmap: (a -> b) -> f a -> f b

class Applicative f : Functor f =
  pure: a -> f a
  liftA2: (a -> b -> c) -> f a -> f b -> f c

class Monad m : Applicative m =
  return: a -> m a
