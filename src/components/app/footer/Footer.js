import React from 'react';
import './Footer.css';

const Footer = () => {
    return (
        <footer className="footer-thin">
            <div className="content has-text-centered">
                <a href="https://github.com/aellwein/beercalc">Project on GitHub</a> | Created by <a href="https://github.com/aellwein">Alex Ellwein</a> | Licensed under the <a href="/license.html">MIT License</a>
            </div>
        </footer>
    );
}

export default Footer;