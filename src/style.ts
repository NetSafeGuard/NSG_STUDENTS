import styled from 'styled-components';
import bg from './assets/wallpaper.png';

export const Container = styled.div`
    display: flex;
    align-items: center;
    height: calc(100vh - 32px);
    background-color: #f6f8fb;
    color: white;
`;

export const Left = styled.div`
    display: flex;
    width: 50vw;
    height: 100%;
    background-position: left;
    background-size: cover;
    background-image: url(${bg});
`;

export const Right = styled.div`
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    height: 100%;
    margin: 0 120px;
    padding-top: 10%;

    @media (max-width: 1060px) {
        margin: 0 60px;
    }
`;

export const Title = styled.h1`
    font-size: 23px;
    font-weight: 700;
    margin-bottom: 20px;
    color: #333;
    font-weight: 500;
`;

export const Form = styled.form`
    display: flex;
    flex-direction: column;
`;

export const Label = styled.label`
    font-size: 13px;
    margin-bottom: 5px;
    color: #181e25;
    margin-bottom: 10px;
    display: flex;
`;

export const Input = styled.input`
    padding: 10px;
    width: 100%;
    margin-bottom: 10px;
    border-radius: 5px;
    border: 1px solid #ccc;
    color: black;
    font-size: 14px;

    &::placeholder {
        font-size: 12px;
        color: #ccc;
    }

    &:focus {
        outline: none;
        border-color: #37505C;
    }

    margin-bottom: 20px;
`;

export const Button = styled.button`
    padding: 10px;
    width: 100%;
    border-radius: 5px;
    border: none;
    background: #015593;
    color: white;
    font-size: 16px;
    cursor: pointer;

    transition: background-color 0.5s;
    &:hover {
        background-color: #016593;
    }
`;

export const Credits = styled.div`
    position: fixed;
    bottom: 10px;
    right: 10px;
    font-size: 12px;
    color: #333;
    opacity: 0.5;
`;