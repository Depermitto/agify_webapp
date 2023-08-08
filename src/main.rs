mod counter;
mod agify;


fn main() {
    yew::Renderer::<agify::Agify>::new().render();
}